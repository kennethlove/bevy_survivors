mod components;
mod constants;
mod enemy;
mod menu;
mod pawn;
mod ui;
mod weapon;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
    window::{Window, WindowTheme},
};
use components::*;
use constants::*;
use enemy::EnemyBundle;
use menu::*;
use pawn::PawnBundle;
use ui::*;
use weapon::WeaponBundle;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum AppState {
    #[default]
    Setup,
    Menu,
    InGame,
}

#[derive(Resource)]
struct Scoreboard {
    score: u32,
    kills: u32,
}

#[derive(Event)]
pub enum ScoreEvent {
    Scored(u32),
    EnemyHit,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Scoreboard { score: 0, kills: 0 })
        .init_state::<AppState>()
        .add_event::<ScoreEvent>()
        .add_event::<MenuEvent>()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        prevent_default_event_handling: false,
                        resizable: false,
                        resolution: Vec2 {
                            x: WIDTH,
                            y: HEIGHT,
                        }
                        .into(),
                        title: "Survivors".into(),
                        window_theme: Some(WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, (setup_camera, setup_background))
        .add_systems(OnEnter(AppState::Menu), (setup_title, setup_menu))
        .add_systems(OnExit(AppState::Menu), (cleanup_title, cleanup_menu))
        .add_systems(OnExit(AppState::InGame), cleanup_ui)
        .add_systems(
            OnExit(AppState::Menu),
            (
                PawnBundle::setup_sprite,
                WeaponBundle::setup_sprite.after(PawnBundle::setup_sprite),
                setup_ui,
                setup_hp.after(WeaponBundle::setup_sprite),
            ),
        )
        .add_systems(
            Update,
            (
                animate_sprites,
                menu_button_system.run_if(in_state(AppState::Menu)),
                draw_border,
                PawnBundle::update_score,
                update_ui
                    .after(PawnBundle::update_score)
                    .run_if(in_state(AppState::InGame)),
                update_hp
                    .after(PawnBundle::update_score)
                    .run_if(in_state(AppState::InGame)),
            ),
        )
        .add_systems(
            FixedUpdate,
            ((
                PawnBundle::collisions,
                PawnBundle::move_pawn,
                WeaponBundle::move_weapon,
                EnemyBundle::move_enemies,
                EnemyBundle::update_enemies,
                EnemyBundle::spawn_enemies,
            )
                .run_if(in_state(AppState::InGame)),),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    let title_font: Handle<Font> = asset_server.load("fonts/DungeonFont.ttf");
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                top: Val::Px(-100.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Survivors".to_string(),
                    TextStyle {
                        font_size: 100.0,
                        color: Color::WHITE,
                        font: title_font,
                    },
                )
                .with_text_justify(JustifyText::Center),
                UI_LAYER,
                TitleText,
            ));
        });
}

fn cleanup_title(mut commands: Commands, query: Query<Entity, With<Text>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let canvas_size = Extent3d {
        width: WIDTH as u32,
        height: HEIGHT as u32,
        ..default()
    };

    // This Image serves as a canvas representing the UI layer
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    canvas.resize(canvas_size);
    let image_handle = images.add(canvas);

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        UICamera,
        UI_LAYER,
    ));

    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn draw_border(mut gizmos: Gizmos) {
    if DRAW_GIZMOS {
        let safe_area = Vec2::new(SAFE_WIDTH, SAFE_HEIGHT);
        gizmos.rect_2d(Vec2::ZERO, 0., safe_area, Color::WHITE);
    }
}

fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("floors/floor_1.png"),
            transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
                ..default()
            },
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 1.,
        },
    ));

    next_state.set(AppState::Menu);
}

fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
