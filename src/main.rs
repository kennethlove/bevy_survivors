mod components;
mod constants;
mod enemy;
mod menu;
mod pawn;
mod ui;
mod weapon;

use std::time::Duration;

use bevy::{
    asset::AssetMetaCheck, audio, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*, render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    }, window::{Window, WindowTheme}
};
use bevy_ecs_tilemap::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_pkv::PkvStore;
use components::*;
use constants::*;
use enemy::EnemyBundle;
use menu::*;
use pawn::PawnBundle;
use serde::{Deserialize, Serialize};
use ui::*;
use weapon::WeaponBundle;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum AppState {
    #[default]
    Setup,
    MainMenu,
    OptionMenu,
    InGame,
    GameOver,
}

#[derive(Resource)]
struct Scoreboard {
    score: u32,
    kills: u32,
}

#[derive(Resource)]
pub struct BackgroundMusic;

#[derive(Resource)]
pub struct SoundFX;

#[derive(Serialize, Deserialize, Debug)]
struct HighScore {
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
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Scoreboard { score: 0, kills: 0 })
        .insert_resource(PkvStore::new("kennethlove", "Survivors"))
        .init_state::<AppState>()
        .add_audio_channel::<BackgroundMusic>()
        .add_audio_channel::<SoundFX>()
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
            // FrameTimeDiagnosticsPlugin,
            // LogDiagnosticsPlugin::default(),
            AudioPlugin,
            TilemapPlugin,
        ))
        .add_systems(Startup, (setup_camera, setup_background, setup_music))
        .add_systems(OnEnter(AppState::MainMenu), (setup_title, setup_main_menu))
        .add_systems(
            OnExit(AppState::MainMenu),
            (cleanup_title, cleanup_main_menu),
        )
        .add_systems(OnExit(AppState::InGame), cleanup_ui)
        .add_systems(
            OnEnter(AppState::InGame),
            (
                PawnBundle::setup_sprite,
                WeaponBundle::setup_sprite.after(PawnBundle::setup_sprite),
                setup_ui,
                setup_hp.after(PawnBundle::setup_sprite),
            ),
        )
        .add_systems(
            OnExit(AppState::InGame),
            (
                PawnBundle::cleanup_sprite,
                WeaponBundle::cleanup_sprite.after(PawnBundle::cleanup_sprite),
                EnemyBundle::cleanup_sprites,
                cleanup_hp,
            ),
        )
        .add_systems(OnEnter(AppState::GameOver), setup_game_over)
        .add_systems(OnExit(AppState::GameOver), (cleanup_game_over, reset))
        .add_systems(
            FixedUpdate,
            ((
                PawnBundle::collisions,
                EnemyBundle::move_enemies,
                EnemyBundle::update_enemies,
                EnemyBundle::spawn_enemies,
                PawnBundle::move_pawn,
                WeaponBundle::move_weapon.after(PawnBundle::move_pawn),
            )
                .run_if(in_state(AppState::InGame)),),
        )
        .add_systems(Update, (
            audio_system,
            main_menu_button_system,
            (
                move_camera,
                animate_sprites,
                PawnBundle::update_score,
                update_ui,
                update_hp,
                pause,
                game_over_button_system,
            ).run_if(in_state(AppState::InGame))
        ))
        // .add_systems(Update, bevy::window::close_on_esc)
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

fn move_camera(
    mut query: Query<(&mut Transform, &MainCamera), Without<Pawn>>,
    pawn_query: Query<&Transform, With<Pawn>>,
) {
    if pawn_query.iter().count() == 0 {
        return;
    }
    let mut camera = query.single_mut();
    camera.0.translation = pawn_query.single().translation.truncate().extend(10.0);
}

fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: Res<
        ArrayTextureLoader,
    >,
) {
    let texture_handle: Handle<Image> = asset_server.load("floors/tiles.png");
    let map_size = TilemapSize { x: 640, y: 320 };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16., y: 16. };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    {
        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(asset_server.load("tiles.png")),
            tile_size,
            ..Default::default()
        });
    }

    next_state.set(AppState::MainMenu);
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

fn pause(mut state: ResMut<NextState<AppState>>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        state.set(AppState::MainMenu);
    }
}

#[derive(Resource)]
struct AudioHandle(Handle<AudioInstance>);


fn audio_system(
    state: Res<State<AppState>>,
    mut bg: ResMut<Assets<AudioInstance>>,
    sfx: ResMut<AudioChannel<SoundFX>>,
    handle: Res<AudioHandle>,
) {
    if let Some(instance) = bg.get_mut(&handle.0) {
        match state.get() {
            AppState::InGame => {
                instance.set_volume(0.5, AudioTween::new(Duration::from_secs(2), AudioEasing::Linear));
                sfx.resume().fade_in(AudioTween::new(Duration::from_secs(2), AudioEasing::Linear));
            }
            _ => {
                instance.set_volume(0.1, AudioTween::new(Duration::from_secs(2), AudioEasing::Linear));
                sfx.pause().fade_out(AudioTween::new(Duration::from_secs(2), AudioEasing::Linear));
            }
        }
    }
}

fn setup_music(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    // background.play(asset_server.load("music/Arcade.ogg")).with_volume(0.5).looped();
    let handle = audio.play(asset_server.load("music/Arcade.ogg")).with_volume(0.5).looped().handle();
    commands.insert_resource(AudioHandle(handle));
    // commands.spawn((
    //     AudioBundle {
    //         source: asset_server.load("music/Arcade.ogg"),
    //         settings: PlaybackSettings::LOOP,
    //     },
    //     BackgroundMusic,
    // ));
}

fn reset(mut scoreboard: ResMut<Scoreboard>) {
    scoreboard.score = 0;
    scoreboard.kills = 0;
}
