use bevy::window::{Window, WindowTheme};
use bevy::{animation, prelude::*};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
}

#[derive(Component)]
struct Pawn;

#[derive(Clone, Component, Debug)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

const IDLE_ANIMATION: AnimationIndices = AnimationIndices { first: 1, last: 5 };
const RUN_ANIMATION: AnimationIndices = AnimationIndices { first: 6, last: 12 };
const ATTACK_ANIMATION: AnimationIndices = AnimationIndices {
    first: 13,
    last: 15,
};
const HEAVY_ATTACK_ANIMATION: AnimationIndices = AnimationIndices {
    first: 16,
    last: 19,
};

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
        .add_systems(FixedUpdate, move_sprite)
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
    let texture = asset_server.load("Warrior_Purple.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(192., 192.), 49, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = IDLE_ANIMATION;

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 1.))
                .with_scale(Vec3::splat(1.)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Pawn,
    ));
}

fn move_sprite(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut Transform,
            &mut AnimationIndices,
            &mut TextureAtlas,
            &mut Sprite,
        ),
        With<Pawn>,
    >,
) {
    for (mut transform, mut animation_indices, mut atlas, mut sprite) in &mut query {
        let mut new_animation_indices = IDLE_ANIMATION;
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.;
            sprite.flip_x = true;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.;
            sprite.flip_x = false;
        }
        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * 100. * time.delta_seconds();
            new_animation_indices = RUN_ANIMATION;
        }

        if keyboard_input.pressed(KeyCode::Space) {
            new_animation_indices = ATTACK_ANIMATION;

            if keyboard_input.pressed(KeyCode::ShiftLeft) {
                new_animation_indices = HEAVY_ATTACK_ANIMATION;
            }
        }

        animation_indices.first = new_animation_indices.first;
        animation_indices.last = new_animation_indices.last;
        if atlas.index > animation_indices.last || atlas.index < animation_indices.first {
            atlas.index = animation_indices.first;
        }
    }
}
