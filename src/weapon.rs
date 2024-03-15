use crate::{AppState, components::*, CollisionEvent, SoundFX, SPRITE_HEIGHT, SPRITE_WIDTH};
use bevy::math::bounding::IntersectsVolume;
use bevy::{
    math::bounding::{Aabb2d, BoundingCircle},
    prelude::*,
};
use bevy_kira_audio::prelude::*;

const STARTING_POSITION: Vec3 = Vec3::ZERO;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), setup_sprite)
            .add_systems(OnExit(AppState::InGame), cleanup_sprite)
            .add_systems(FixedUpdate, (move_weapon, collide_enemies.after(move_weapon)).run_if(in_state(AppState::InGame)));
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
    sfx: Res<AudioChannel<SoundFX>>,
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

    commands.spawn(WeaponBundle {
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
    });
    sfx.play(audio).looped();
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

pub fn collide_enemies(
    mut weapon_query: Query<(&mut AnimationTimer, &TextureAtlas, &Transform, &Weapon)>,
    mut enemies: Query<(Entity, &Transform, &mut Sprite), With<Enemy>>,
    time: Res<Time>,
    mut events: EventWriter<CollisionEvent>,
    mut gizmos: Gizmos,
) {
    if weapon_query.is_empty() {
        return;
    }

    let (mut weapon_timer, weapon_atlas, weapon_transform, weapon) = weapon_query.single_mut();
    let weapon_radius = weapon_transform.scale.x * (SPRITE_WIDTH as f32 * 2.);
    let weapon_circle =
        BoundingCircle::new(weapon_transform.translation.truncate(), weapon_radius);
    gizmos.circle_2d(weapon_circle.center, weapon_circle.radius(), Color::RED);
    for (entity, &transform, mut sprite) in &mut enemies {
        sprite.color = Color::WHITE;
        let enemy_rect = Rect::from_center_size(
            transform.translation.truncate(),
            Vec2::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32) * transform.scale.truncate(),
        );
        let enemy_aabb = Aabb2d::new(enemy_rect.center(), enemy_rect.half_size());
        let collision = weapon_circle.intersects(&enemy_aabb);
        if collision
            && weapon_timer.0.tick(time.delta()).just_finished()
            && weapon_atlas.index <= weapon.damage_frame_end
            && weapon_atlas.index >= weapon.damage_frame_start
        {
            events.send(CollisionEvent::WeaponHitsEnemy(entity));
        }
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
