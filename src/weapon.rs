use crate::components::{AnimationIndices, AnimationTimer, Enemy};
use crate::constants::*;
use crate::CollisionEvent;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

const STARTING_POSITION: Vec3 = Vec3::ZERO;

#[derive(Component)]
pub struct Weapon {
    damage_amount: u32,
    damage_frame: usize,
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
                damage_amount: 50,
                damage_frame: 0,
            },
        }
    }
}

impl WeaponBundle {
    pub fn setup_sprite(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let texture = asset_server.load("super/03.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(32., 32.), 6, 2, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 11 };

        let mut transform = Transform::from_translation(STARTING_POSITION);
        transform.translation.z = -1.;
        transform.scale = Vec3::splat(4.);

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
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            ..default()
        });
    }

    pub fn move_weapon(
        time: Res<Time>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut query: Query<&mut Transform, With<Weapon>>,
    ) {
        for mut transform in &mut query {
            let mut direction = Vec3::ZERO;
            if keyboard_input.pressed(KeyCode::KeyW) {
                direction.y += 1.;
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                direction.y -= 1.;
            }
            if keyboard_input.pressed(KeyCode::KeyA) {
                direction.x -= 1.;
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                direction.x += 1.;
            }
            if direction != Vec3::ZERO {
                let mut speed = PAWN_SPEED;
                if keyboard_input.pressed(KeyCode::ShiftLeft)
                    || keyboard_input.pressed(KeyCode::ShiftRight)
                {
                    speed = PAWN_SPEED_FAST;
                }

                let mut new_translation =
                    transform.translation + direction.normalize() * speed * time.delta_seconds();

                new_translation.x = new_translation.x.clamp(-SAFE_WIDTH / 2., SAFE_WIDTH / 2.);
                new_translation.y = new_translation.y.clamp(-SAFE_HEIGHT / 2., SAFE_HEIGHT / 2.);

                transform.translation = Vec3::new(new_translation.x, new_translation.y, 2.);
            }
        }
    }

    pub fn attack(
        mut weapon_query: Query<(&mut AnimationTimer, &TextureAtlas, &Transform, &Weapon)>,
        mut enemy_query: Query<(Entity, &Transform)>,
        time: Res<Time>,
        mut events: EventWriter<CollisionEvent>,
    ) {
        let (mut weapon_timer, weapon_atlas, weapon_transform, weapon) = weapon_query.single_mut();

        for (entity, enemy_transform) in &mut enemy_query {
            let enemy_rect = Rect::from_center_size(
                enemy_transform.translation.truncate(),
                Vec2::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32)
                    * enemy_transform.scale.truncate(),
            );
            let enemy_pos = Aabb2d::new(enemy_rect.center(), enemy_rect.size());

            let weapon_rect = Rect::from_center_size(
                weapon_transform.translation.truncate(),
                Vec2::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32)
                    * weapon_transform.scale.truncate(),
            );
            let weapon_pos = Aabb2d::new(weapon_rect.center(), weapon_rect.size());

            let collision = weapon_pos.intersects(&enemy_pos);
            if collision
                && weapon_timer.0.tick(time.delta()).just_finished()
                && weapon_atlas.index == weapon.damage_frame
            {
                events.send(CollisionEvent {
                    entity,
                    amount: weapon.damage_amount,
                });
            }
        }
    }
}
