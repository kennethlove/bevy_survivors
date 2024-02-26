mod components;
mod constants;
mod enemy;
mod pawn;

use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
    transform::commands,
    window::{CursorGrabMode, Window, WindowTheme},
};
use components::*;
use constants::*;
use enemy::EnemyBundle;
use pawn::PawnBundle;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum AppState {
    #[default]
    Setup,
    Menu,
    InGame,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
        .init_state::<AppState>()
        .add_plugins(
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
        )
        .add_systems(Startup, (setup_camera, setup_background))
        .add_systems(OnEnter(AppState::Menu), (setup_title, setup_menu))
        .add_systems(OnExit(AppState::Menu), (cleanup_title, cleanup_menu))
        .add_systems(OnExit(AppState::Menu), PawnBundle::setup_sprite)
        .add_systems(
            Update,
            (
                // grab_mouse,
                animate_sprites,
                menu_button_system.run_if(in_state(AppState::Menu)),
                draw_border,
            ),
        )
        .add_systems(
            FixedUpdate,
            ((
                PawnBundle::move_sprite,
                PawnBundle::collisions,
                EnemyBundle::spawn_enemies,
                EnemyBundle::update_enemies,
            )
                .run_if(in_state(AppState::InGame)),),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn menu_button_system(
    mut state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiImage),
        (Changed<Interaction>, With<Button>),
    >,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let pressed_image = asset_server.load("buttons/Button_Blue_9Slides_Pressed.png");
    let regular_image = asset_server.load("buttons/Button_Blue_9Slides.png");
    let hover_image = asset_server.load("buttons/Button_Hover_9Slides.png");
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(AppState::InGame);
    }
    for (interaction, mut ui_image) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                ui_image.texture = pressed_image.clone();
                state.set(AppState::InGame)
            }
            Interaction::Hovered => {
                ui_image.texture = hover_image.clone();
            }
            _ => {
                ui_image.texture = regular_image.clone();
            }
        }
    }
}

fn setup_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    let title_font: Handle<Font> = asset_server.load("fonts/DungeonFont.ttf");
    commands.spawn((
        TextBundle::from_section(
            "Survivors".to_string(),
            TextStyle {
                font_size: 100.0,
                color: Color::WHITE,
                font: title_font,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(100.),
            left: Val::Px(100.),
            ..default()
        }),
        UI_LAYER,
        TitleText,
    ));
}

fn cleanup_title(mut commands: Commands, query: Query<Entity, With<Text>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("buttons/Button_Blue_9Slides.png");

    let slicer = TextureSlicer {
        border: BorderRect::square(22.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.,
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(100.),
                            height: Val::Px(25.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(20.)),
                            ..default()
                        },
                        image: image.clone().into(),
                        ..default()
                    },
                    ImageScaleMode::Sliced(slicer.clone()),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Play".to_string(),
                            TextStyle {
                                font_size: 20.,
                                color: Color::DARK_GRAY,
                                ..default()
                            },
                        ),
                        UI_LAYER,
                    ));
                });
        });
}

fn cleanup_menu(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction, &mut UiImage), With<Button>>,
) {
    for entity in &mut interaction_query.iter() {
        commands.entity(entity.0).despawn_recursive();
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

fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
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
