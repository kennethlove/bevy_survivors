use bevy::prelude::*;
use bevy::window::{Window, WindowTheme};
use bevy_ase::asset::AseAsset;
use bevy_ase::loader::{AseLoaderDefaultPlugin, Loader};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
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
                        resolution: Vec2 { x: 800., y: 600. }.into(),
                        title: "Survivors".into(),
                        window_theme: Some(WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, (setup_camera, setup_background, setup_sprite))
        // .add_systems(OnEnter(AppState::Finished), setup_player)
        .add_systems(Update, animate_sprite)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("floors/floor_1.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::from((800., 600.))),
                ..default()
            },
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 2.,
        },
    ));
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("knight/knight_f_idle_anim_f0.png"),
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        sprite: Sprite {
            custom_size: Some(Vec2::from((16., 28.))),
            ..default()
        },
        ..default()
    });
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
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

fn setup_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("character_sheet.png");
    let layout =
        TextureAtlasLayout::from_grid(Vec2::new(12., 12.), 4, 4, Some(Vec2::new(2., 2.)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 16 };

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            // transform: Transform::from_scale(Vec3::splat(6.)),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.))
                .with_scale(Vec3::splat(2.)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
