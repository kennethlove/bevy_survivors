use crate::components::*;
use crate::constants::*;
use crate::weapon::Weapon;
use crate::{ScoreEvent, Scoreboard};
use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::*;

const IDLE_ANIMATION: AnimationIndices = AnimationIndices { first: 0, last: 1 };
const RUN_ANIMATION: AnimationIndices = AnimationIndices { first: 0, last: 1 };

#[derive(Component, Clone)]
pub struct EnemySprite {
    filename: String,
    layout: TextureAtlasLayout,
    idle: AnimationIndices,
    run: AnimationIndices,
    speed: f32,
    pub height: f32,
    pub width: f32,
    health: f32,
    score: f32,
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
            pawn: Enemy,
            sprite_details: EnemySprite {
                filename: "16x32.png".to_string(),
                layout: TextureAtlasLayout::new_empty(Vec2::ZERO),
                idle: IDLE_ANIMATION,
                run: RUN_ANIMATION,
                height: 16.,
                width: 16.,
                speed: 0.1,
                health: 1.,
                score: 1.,
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
        let min_x: usize = (player_pos.x + (WIDTH / 2.)).trunc() as usize;
        let max_x: usize = min_x + (SPRITE_WIDTH * 2) as usize;

        let max_y: usize = (HEIGHT / 2.).trunc() as usize + (SPRITE_HEIGHT * 2) as usize;

        let mut x = fastrand::usize(min_x..max_x) as isize;
        let mut y = fastrand::usize(..max_y) as isize;

        if fastrand::bool() {
            x = -x;
        }
        if fastrand::bool() {
            y = -y;
        }

        Vec3::new(x as f32, y as f32, 0.)
    }

    pub fn spawn_enemies(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
        enemies: Query<&Transform, With<Enemy>>,
        player: Query<&Transform, With<Pawn>>,
        scoreboard: Res<Scoreboard>,
    ) {
        let count = enemies.iter().count();

        let green_kobold = EnemySprite {
            filename: String::from("enemies/green_kobold.png"),
            layout: TextureAtlasLayout::from_grid(Vec2::new(16., 20.), 8, 1, None, None),
            idle: AnimationIndices { first: 0, last: 1 },
            run: AnimationIndices { first: 0, last: 7 },
            width: 16.,
            height: 20.,
            speed: 0.3,
            health: 100.,
            score: 100.,
        };

        let blue_kobold = EnemySprite {
            filename: String::from("enemies/blue_kobold.png"),
            layout: TextureAtlasLayout::from_grid(Vec2::new(16., 20.), 8, 1, None, None),
            idle: AnimationIndices { first: 0, last: 1 },
            run: AnimationIndices { first: 0, last: 7 },
            width: 16.,
            height: 20.,
            speed: 0.3,
            health: 200.,
            score: 200.,
        };

        let troll = EnemySprite {
            filename: String::from("enemies/troll.png"),
            layout: TextureAtlasLayout::from_grid(
                Vec2::new(48., 38.),
                12,
                1,
                Some(Vec2::new(16., 0.)),
                None,
            ),
            idle: AnimationIndices { first: 0, last: 1 },
            run: AnimationIndices { first: 0, last: 11 },
            width: 48.,
            height: 38.,
            speed: 0.1,
            health: 500.,
            score: 1000.,
        };

        let good_spot = EnemyBundle::find_good_spot(enemies, player);

        if count < ((scoreboard.kills + 1) * 100) as usize {
            let enemy = match scoreboard.kills {
                0..=50 => green_kobold.clone(),
                51..=75 => blue_kobold.clone(),
                76..=100 => troll.clone(),
                _ => green_kobold.clone(),
            };

            let texture: Handle<Image> = asset_server.load(&enemy.filename);
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
                        layout: texture_atlas_layouts.add(layout),
                        index: animation_indices.first,
                    },
                    ..default()
                },
                animation_indices,
                pawn: Enemy,
                sprite_details: enemy.clone(),
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
            transform.translation += direction * sprite_details.speed;

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
        mut enemies: Query<(Entity, &mut EnemySprite, &Transform, &mut Sprite), Without<Pawn>>,
        time: Res<Time>,
        mut events: EventWriter<ScoreEvent>,
        mut gizmos: Gizmos,
        asset_server: Res<AssetServer>,
    ) {
        let (mut weapon_timer, weapon_atlas, weapon_transform, weapon) = weapon_query.single_mut();
        let weapon_circle = BoundingCircle::new(
            weapon_transform.translation.truncate(),
            weapon_transform.scale.x * SPRITE_WIDTH as f32,
        );
        if DRAW_GIZMOS {
            gizmos.circle_2d(weapon_circle.center, weapon_circle.radius(), Color::YELLOW);
        }

        for (entity, mut enemy, &transform, mut sprite) in &mut enemies {
            sprite.color = Color::WHITE;
            let enemy_rect = Rect::from_center_size(
                transform.translation.truncate(),
                Vec2::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32) * transform.scale.truncate(),
            );
            let enemy_aabb = Aabb2d::new(enemy_rect.center(), enemy_rect.size());
            let collision = weapon_circle.intersects(&enemy_aabb);
            if collision
                && weapon_timer.0.tick(time.delta()).just_finished()
                && weapon_atlas.index <= weapon.damage_frame_end
                && weapon_atlas.index >= weapon.damage_frame_start
            {
                let health = enemy.health - weapon.damage_amount * weapon.damage_scale;
                if health <= 0. {
                    commands.get_entity(entity).unwrap().despawn();
                    events.send(ScoreEvent::Scored(enemy.score as u32));
                    commands.spawn((
                        AudioBundle {
                            source: asset_server.load("sfx/enemy_death.ogg"),
                            ..default()
                        },
                        SFX,
                    ));
                } else {
                    sprite.color = Color::RED;
                    enemy.health = health;
                    events.send(ScoreEvent::EnemyHit);
                }
            }
        }
    }

    pub fn cleanup_sprites(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
        for entity in &query {
            commands.entity(entity).despawn();
        }
    }
}
