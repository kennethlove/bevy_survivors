use crate::components::{AnimationIndices, AnimationTimer, Enemy, Pawn};
use crate::constants::*;
use crate::weapon::Weapon;
use crate::ScoreEvent;
use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::*;

const IDLE_ANIMATION: AnimationIndices = AnimationIndices {
    first: 112,
    last: 113,
};
const RUN_ANIMATION: AnimationIndices = AnimationIndices {
    first: 112,
    last: 119,
};

#[derive(Component, Clone)]
pub struct EnemySprite {
    sprite: String,
    layout: Handle<TextureAtlasLayout>,
    idle: AnimationIndices,
    run: AnimationIndices,
    pub height: f32,
    pub width: f32,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub sprite: SpriteSheetBundle,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub pawn: Enemy,
    pub sprite_details: EnemySprite,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        EnemyBundle {
            sprite: SpriteSheetBundle::default(),
            animation_indices: IDLE_ANIMATION,
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            pawn: Enemy {
                health: 1,
                score: 1,
            },
            sprite_details: EnemySprite {
                sprite: "16x32.png".to_string(),
                layout: Handle::default(),
                idle: IDLE_ANIMATION,
                run: RUN_ANIMATION,
                height: 16.,
                width: 16.,
            },
        }
    }
}

impl EnemyBundle {
    fn find_good_spot(
        _enemies: Query<&Transform, With<Enemy>>,
        player: Query<&Transform, With<Pawn>>,
    ) -> Vec3 {
        let player_pos = player.single().translation;

        let mut x = fastrand::i32(..) % SAFE_WIDTH as i32;
        if fastrand::bool() {
            x = -x;
        }
        x = x.clamp(
            ((-SAFE_WIDTH + SPRITE_WIDTH as f32) / 2.) as i32,
            ((SAFE_WIDTH - SPRITE_WIDTH as f32) / 2.) as i32,
        );

        let mut y = fastrand::i32(..) % SAFE_HEIGHT as i32;
        if fastrand::bool() {
            y = -y;
        }
        y = y.clamp(
            ((-SAFE_HEIGHT + SPRITE_HEIGHT as f32) / 2.) as i32,
            ((SAFE_HEIGHT - SPRITE_HEIGHT as f32) / 2.) as i32,
        );

        let translation = Vec3::new(x as f32, y as f32, 0.);
        let sprite_area = Vec2::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32);
        if Aabb2d::new(translation.truncate(), sprite_area)
            .intersects(&Aabb2d::new(player_pos.truncate(), sprite_area))
        {
            return EnemyBundle::find_good_spot(_enemies, player);
        }
        translation
    }

    pub fn spawn_enemies(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
        enemies: Query<&Transform, With<Enemy>>,
        player: Query<&Transform, With<Pawn>>,
    ) {
        let count = enemies.iter().count();
        let good_spot = EnemyBundle::find_good_spot(enemies, player);

        let green_kobold = EnemySprite {
            sprite: "16x32.png".to_string(),
            layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                Vec2::new(16., 32.),
                8,
                64,
                None,
                None,
            )),
            idle: IDLE_ANIMATION,
            run: RUN_ANIMATION,
            width: 16.,
            height: 32.,
        };

        if count < 2 {
            let mut enemy = green_kobold.clone();
            let texture: Handle<Image> = asset_server.load(&enemy.sprite);
            let layout = enemy.layout.clone();
            let animation_indices = AnimationIndices {
                first: enemy.idle.first,
                last: enemy.idle.last,
            };
            let mut transform = Transform::from_translation(good_spot);
            transform = transform.with_scale(Vec3::splat(2.));

            commands.spawn(EnemyBundle {
                sprite: SpriteSheetBundle {
                    texture,
                    transform, // Controls the placement of the sprite
                    atlas: TextureAtlas {
                        layout,
                        index: animation_indices.first,
                    },
                    ..default()
                },
                animation_indices,
                pawn: Enemy {
                    health: 100,
                    score: 20,
                },
                sprite_details: enemy.clone(),
                ..default()
            });

            let ogre = EnemySprite {
                sprite: "48x48.png".to_string(),
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    Vec2::new(48., 48.),
                    6,
                    2,
                    Some(Vec2::new(15., 15.)),
                    None,
                )),
                idle: AnimationIndices { first: 0, last: 1 },
                run: AnimationIndices { first: 0, last: 11 },
                width: 48.,
                height: 48.,
            };

            commands.spawn(EnemyBundle {
                sprite: SpriteSheetBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(ogre.width, ogre.height)),
                        ..default()
                    },
                    texture: asset_server.load(ogre.sprite.clone()),
                    atlas: TextureAtlas {
                        layout: ogre.layout.clone(),
                        index: ogre.run.first,
                    },
                    ..default()
                },
                animation_indices: ogre.run.clone(),
                pawn: Enemy {
                    health: 1000,
                    score: 2000,
                },
                sprite_details: ogre.clone(),
                ..default()
            });
        }
    }

    pub fn move_enemies(
        mut params: ParamSet<(
            Query<&Transform, With<Pawn>>,
            Query<
                (
                    &mut Transform,
                    &mut AnimationIndices,
                    &mut TextureAtlas,
                    &mut Sprite,
                    &EnemySprite,
                ),
                With<Enemy>,
            >,
        )>,
    ) {
        let player_pos = params.p0().single().translation;
        for (mut transform, mut animation_indices, mut atlas, mut sprite, sprite_details) in
            &mut params.p1()
        {
            let mut direction = player_pos - transform.translation;
            direction = direction.normalize();
            sprite.flip_x = direction.x < 0.;
            transform.translation += direction * ENEMY_SPEED;

            let new_animation_indices = AnimationIndices {
                first: sprite_details.run.first,
                last: sprite_details.run.last,
            };

            animation_indices.first = new_animation_indices.first;
            animation_indices.last = new_animation_indices.last;
            if atlas.index > animation_indices.last || atlas.index < animation_indices.first {
                atlas.index = animation_indices.first;
            }
        }
    }

    pub fn update_enemies(
        mut commands: Commands,
        mut weapon_query: Query<(&mut AnimationTimer, &TextureAtlas, &Transform, &Weapon)>,
        mut enemies: Query<(Entity, &mut Enemy, &Transform), Without<Pawn>>,
        time: Res<Time>,
        mut events: EventWriter<ScoreEvent>,
        mut gizmos: Gizmos,
    ) {
        let (mut weapon_timer, weapon_atlas, weapon_transform, weapon) = weapon_query.single_mut();
        let weapon_circle = BoundingCircle::new(
            weapon_transform.translation.truncate(),
            weapon_transform.scale.x * SPRITE_WIDTH as f32,
        );
        gizmos.circle_2d(weapon_circle.center, weapon_circle.radius(), Color::YELLOW);

        for (entity, mut enemy, &transform) in &mut enemies {
            let enemy_rect = Rect::from_center_size(
                transform.translation.truncate(),
                Vec2::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32) * transform.scale.truncate(),
            );
            let enemy_aabb = Aabb2d::new(enemy_rect.center(), enemy_rect.size());
            let collision =
                weapon_circle.intersects(&enemy_aabb) && enemy_aabb.intersects(&weapon_circle);
            if collision
                && weapon_timer.0.tick(time.delta()).just_finished()
                && weapon_atlas.index <= weapon.damage_frame_end
                && weapon_atlas.index >= weapon.damage_frame_start
            {
                let health = enemy.health as f32 - weapon.damage_amount * weapon.damage_scale;
                let health = std::cmp::max(0, health as i32);
                if health == 0 {
                    commands.get_entity(entity).unwrap().despawn();
                    events.send(ScoreEvent::Scored(enemy.score));
                } else {
                    enemy.health = health as u32;
                    events.send(ScoreEvent::EnemyHit);
                }
            }
        }
    }
}
