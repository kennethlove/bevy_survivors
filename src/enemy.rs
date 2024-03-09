use crate::components::*;
use crate::constants::*;
use crate::Attack;
use crate::CollisionEvent;
use crate::{ScoreEvent, Scoreboard, SoundFX};
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

const IDLE_ANIMATION: AnimationIndices = AnimationIndices { first: 0, last: 1 };
const RUN_ANIMATION: AnimationIndices = AnimationIndices { first: 0, last: 1 };

#[derive(Component, Clone, Debug)]
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

        let enemy_transform = Transform::from_translation(Vec3::new(x as f32, y as f32, 0.));
        if enemy_transform.translation.distance(player_pos) < WIDTH / 4. {
            return EnemyBundle::find_good_spot(_enemies, player);
        }
        Vec3::new(x as f32, y as f32, 0.)
    }

    fn green_kobold() -> EnemySprite {
        EnemySprite {
            filename: String::from("enemies/green_kobold.png"),
            layout: TextureAtlasLayout::from_grid(Vec2::new(16., 20.), 8, 1, None, None),
            idle: AnimationIndices { first: 0, last: 1 },
            run: AnimationIndices { first: 0, last: 7 },
            width: 16.,
            height: 20.,
            speed: 0.3,
            health: 100.,
            score: 100.,
        }
    }

    fn blue_kobold() -> EnemySprite {
        EnemySprite {
            filename: String::from("enemies/blue_kobold.png"),
            layout: TextureAtlasLayout::from_grid(Vec2::new(16., 20.), 8, 1, None, None),
            idle: AnimationIndices { first: 0, last: 1 },
            run: AnimationIndices { first: 0, last: 7 },
            width: 16.,
            height: 20.,
            speed: 0.3,
            health: 200.,
            score: 200.,
        }
    }

    fn troll() -> EnemySprite {
        EnemySprite {
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
        }
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

        let good_spot = EnemyBundle::find_good_spot(enemies, player);

        if count < ((scoreboard.kills + 1) * 100) as usize {
            let enemy = match scoreboard.kills {
                0..=50 => Self::green_kobold(),
                51..=75 => Self::blue_kobold(),
                76..=100 => Self::troll(),
                _ => Self::green_kobold(),
            };

            let texture: Handle<Image> = asset_server.load(&enemy.filename);
            let layout = enemy.layout.clone();
            let animation_indices = AnimationIndices {
                first: enemy.idle.first,
                last: enemy.idle.last,
            };
            let mut transform = Transform::from_translation(good_spot);
            transform = transform.with_scale(Vec3::splat(1.));

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

    pub fn collide_with_weapon(
        mut commands: Commands,
        mut enemies: Query<(Entity, &mut EnemySprite, &mut Sprite), With<Enemy>>,
        mut events: EventReader<CollisionEvent>,
        mut score_events: EventWriter<ScoreEvent>,
        asset_server: Res<AssetServer>,
        sfx: Res<AudioChannel<SoundFX>>,
        mut attack: Res<Attack>,
    ) {
        for event in events.read() {
            if let CollisionEvent::WeaponHitsEnemy(event_entity) = event {
                let event_entity_id = commands.get_entity(*event_entity).unwrap().id();
                for (entity, mut enemy, mut sprite) in &mut enemies {
                    let entity_id = commands.get_entity(entity).unwrap().id();
                    if entity_id == event_entity_id {
                        sprite.color = Color::WHITE;
                        let health =
                            enemy.health - (attack.damage_amount * attack.damage_scale) as f32;
                        if health <= 0. {
                            commands.get_entity(*event_entity).unwrap().despawn();
                            score_events.send(ScoreEvent::Scored(enemy.score as u32));
                            sfx.play(asset_server.load("sfx/enemy_death.ogg"))
                                .with_volume(0.2);
                        } else {
                            sprite.color = Color::RED;
                            enemy.health = health;
                            score_events.send(ScoreEvent::EnemyHit);
                        }
                    }
                }
            } else {
                continue;
            }
        }
    }

    pub fn collide_with_player(
        mut commands: Commands,
        enemies: Query<(Entity, &Transform, &EnemySprite), With<Enemy>>,
        player: Query<&Transform, With<Pawn>>,
        mut events: EventWriter<CollisionEvent>,
    ) {
        let player_transform = player.single();
        let player_size = Vec2::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32)
            * player_transform.scale.truncate();
        let player_aabb = Aabb2d::new(player_transform.translation.truncate(), player_size / 2.);
        for (entity, transform, sprite) in &mut enemies.iter() {
            let enemy_size = Vec2::new(sprite.width, sprite.height) * transform.scale.truncate();
            let enemy_aabb = Aabb2d::new(transform.translation.truncate(), enemy_size);
            let collision = player_aabb.intersects(&enemy_aabb);
            if collision {
                commands.get_entity(entity).unwrap().despawn();
                events.send(CollisionEvent::EnemyHitsPawn(entity));
            }
        }
    }

    pub fn cleanup_sprites(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
        for entity in &query {
            commands.entity(entity).despawn();
        }
    }
}
