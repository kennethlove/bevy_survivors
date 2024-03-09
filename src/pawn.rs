use crate::components::{AnimationIndices, AnimationTimer, Enemy, Pawn};
use crate::{constants::*, CollisionEvent, MovementEvent};
use crate::enemy::EnemySprite;
use crate::weapon::Weapon;
use crate::AppState;
use crate::{ScoreEvent, Scoreboard};
use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

const IDLE_ANIMATION: AnimationIndices = AnimationIndices { first: 0, last: 1 };
const RUN_ANIMATION: AnimationIndices = AnimationIndices { first: 1, last: 7 };
const STARTING_POSITION: Vec3 = Vec3::ZERO;

#[derive(Bundle)]
pub struct PawnBundle {
    sprite: SpriteSheetBundle,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
    pawn: Pawn,
}

impl Default for PawnBundle {
    fn default() -> Self {
        PawnBundle {
            sprite: SpriteSheetBundle {
                transform: Transform::from_translation(STARTING_POSITION),
                ..default()
            },
            animation_indices: IDLE_ANIMATION,
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            pawn: Pawn {
                speed: PAWN_SPEED,
                health: 1.,
            },
        }
    }
}

impl PawnBundle {
    pub fn setup_sprite(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let texture = asset_server.load("pawns/purple_knight.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(16., 22.), 8, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = IDLE_ANIMATION;

        let mut transform = Transform::from_translation(STARTING_POSITION);
        transform.translation.z = 9.;
        transform.scale = Vec3::splat(2.);

        commands.spawn(PawnBundle {
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
            pawn: Pawn {
                speed: PAWN_SPEED,
                health: 100.,
            },
        });
    }

    pub fn move_pawn(
        time: Res<Time>,
        mut movement_events: EventReader<MovementEvent>,
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
        let (mut transform, mut animation_indices, mut atlas, mut sprite) = query.single_mut();
        let mut new_animation_indices = IDLE_ANIMATION;
        for event in movement_events.read() {
            if let MovementEvent::Move(direction) = event {
                let speed = PAWN_SPEED;
                if direction == &Vec2::ZERO {
                    new_animation_indices = IDLE_ANIMATION;
                } else {
                    if direction.x < 0. {
                        sprite.flip_x = true;
                    } else if direction.x > 0. {
                        sprite.flip_x = false;
                    }
                    new_animation_indices = RUN_ANIMATION;
                }
                if direction != &Vec2::ZERO {
                    let new_translation =
                        transform.translation.truncate() + direction.normalize() * speed * time.delta_seconds();
                    transform.translation = Vec3::new(new_translation.x, new_translation.y, 2.);
                }
            }
            animation_indices.first = new_animation_indices.first;
            animation_indices.last = new_animation_indices.last;
            if atlas.index > animation_indices.last || atlas.index < animation_indices.first {
                atlas.index = animation_indices.first;
            }
        }
    }

    pub fn update_score(mut score: ResMut<Scoreboard>, mut events: EventReader<ScoreEvent>) {
        for event in events.read() {
            match event {
                ScoreEvent::Scored(amount) => {
                    score.score += amount;
                    score.kills += 1;
                }
                ScoreEvent::EnemyHit => {
                    score.score += 10;
                }
            }
        }
    }

    pub fn cleanup_sprite(mut commands: Commands, mut query: Query<Entity, With<Pawn>>) {
        for entity in &mut query {
            commands.entity(entity).despawn();
        }
    }

    pub fn collide_enemies(
        mut events: EventReader<CollisionEvent>,
        mut player_query: Query<&mut Pawn, Without<Enemy>>,
        mut state: ResMut<NextState<AppState>>,
    ) {
        let mut player = player_query.single_mut();
        let mut new_health = player.health.round() as isize;
        for event in events.read() {
            match event {
                CollisionEvent::WithEnemy(_) => {
                    new_health -= 1;
                },
                _ => {}
            }
        }
        if new_health <= 0 {
            state.set(AppState::GameOver);
        } else {
            player.health = new_health as f32;
        }
    }
}
