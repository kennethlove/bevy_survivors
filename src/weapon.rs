use crate::{
    animation::{AnimationIndices, AnimationTimer},
    components::*,
    constants::*,
    AppState, MyCollisionEvent,
};
use bevy::{
    audio::{AudioBundle, PlaybackMode, Volume},
    math::bounding::IntersectsVolume,
};
use bevy::{
    math::bounding::{Aabb2d, BoundingCircle},
    prelude::*,
};
// use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

const STARTING_POSITION: Vec3 = Vec3::ZERO;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_sprite)
            .add_systems(OnExit(AppState::InGame), cleanup_sprite)
            .add_systems(FixedUpdate, move_weapon.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Clone, Component)]
pub struct Weapon {
    audio_filename: String,
    filename: String,
    pub damage_frame_end: usize,
    pub damage_frame_start: usize,
}

#[derive(Bundle)]
pub struct WeaponBundle {
    sprite: SpriteSheetBundle,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
    weapon: Weapon,
}

impl Default for WeaponBundle {
    fn default() -> Self {
        WeaponBundle {
            sprite: SpriteSheetBundle {
                transform: Transform::from_translation(STARTING_POSITION),
                ..default()
            },
            animation_indices: AnimationIndices { first: 0, last: 0 },
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            weapon: Weapon {
                audio_filename: String::from(""),
                filename: String::from(""),
                damage_frame_start: 0,
                damage_frame_end: 0,
            },
        }
    }
}

pub fn setup_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let weapon = weapon_01();
    let texture = asset_server.load(&weapon.filename);
    let layout = TextureAtlasLayout::from_grid(Vec2::new(64., 64.), 8, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 23 };
    let audio = asset_server.load(&weapon.audio_filename);

    let mut transform = Transform::from_translation(STARTING_POSITION);
    transform.translation.z = 0.;
    transform.scale = Vec3::splat(2.);

    commands.spawn((
        WeaponBundle {
            sprite: SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                transform,
                ..default()
            },
            animation_indices,
            animation_timer: AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
            weapon,
        },
        RigidBody::KinematicPositionBased,
        Collider::ball(32.),
        ActiveEvents::COLLISION_EVENTS,
        Sensor,
        SolverGroups::new(ENEMY_WEAPON_GROUP, Group::default()),
        AudioBundle {
            source: audio,
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(0.5),
                ..default()
            },
        },
    ));
}

pub fn move_weapon(
    mut query: Query<(&mut Transform, &Weapon), Without<Pawn>>,
    pawn_query: Query<&Transform, With<Pawn>>,
) {
    for mut transform in &mut query {
        let pawn_transform = pawn_query.single();
        transform.0.translation = pawn_transform.translation.truncate().extend(1.);
    }
}

pub fn cleanup_sprite(mut commands: Commands, mut query: Query<Entity, With<Weapon>>) {
    for entity in &mut query {
        commands.entity(entity).despawn();
    }
}

fn weapon_01() -> Weapon {
    Weapon {
        audio_filename: String::from("sfx/woosh2.ogg"),
        filename: String::from("magic/241.png"),
        damage_frame_start: 4,
        damage_frame_end: 18,
    }
}

fn weapon_02() -> Weapon {
    Weapon {
        audio_filename: String::from("sfx/woosh2.ogg"),
        filename: String::from("magic/242.png"),
        damage_frame_start: 0,
        damage_frame_end: 23,
    }
}

fn weapon_03() -> Weapon {
    Weapon {
        audio_filename: String::from("sfx/woosh2.ogg"),
        filename: String::from("magic/243.png"),
        damage_frame_start: 0,
        damage_frame_end: 23,
    }
}

fn weapon_04() -> Weapon {
    Weapon {
        audio_filename: String::from("sfx/woosh2.ogg"),
        filename: String::from("magic/244.png"),
        damage_frame_start: 0,
        damage_frame_end: 23,
    }
}
