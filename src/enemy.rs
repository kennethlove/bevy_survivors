use crate::animation::{AnimationIndices, AnimationTimer};
use crate::audio::SoundFX;
use crate::collision::EnemyHitPlayer;
use crate::components::*;
use crate::constants::*;
use crate::pawn::Attack;
use crate::AppState;
use crate::MyCollisionEvent;
use crate::{ScoreEvent, Scoreboard};
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

const IDLE_ANIMATION: AnimationIndices = AnimationIndices { first: 0, last: 1 };
const RUN_ANIMATION: AnimationIndices = AnimationIndices { first: 0, last: 1 };

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                spawn_enemies,
                move_enemies,
                // collided_with_weapon,
                collided_with_player,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnExit(AppState::InGame), cleanup_sprites);
    }
}

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
        return find_good_spot(_enemies, player);
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

    let good_spot = find_good_spot(enemies, player);

    if count < ((scoreboard.kills + 1) * 100) as usize {
        let enemy = match scoreboard.kills {
            0..=50 => green_kobold(),
            51..=75 => blue_kobold(),
            76..=100 => troll(),
            _ => green_kobold(),
        };

        let texture: Handle<Image> = asset_server.load(&enemy.filename);
        let layout = enemy.layout.clone();
        let animation_indices = AnimationIndices {
            first: enemy.idle.first,
            last: enemy.idle.last,
        };
        let mut transform = Transform::from_translation(good_spot);
        transform = transform.with_scale(Vec3::splat(1.));

        commands.spawn((
            EnemyBundle {
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
            },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::cuboid(enemy.width / 2., enemy.height / 2.),
            Damping {
                linear_damping: 0.9,
                angular_damping: 0.9,
            },
            Dominance::group(10),
            Friction {
                coefficient: 0.9,
                combine_rule: CoefficientCombineRule::Average,
            },
            Restitution {
                coefficient: 0.1,
                combine_rule: CoefficientCombineRule::Average,
            },
            AdditionalMassProperties::Mass(1.)
        ));
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

pub fn collided_with_weapon(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut EnemySprite, &mut Sprite), With<Enemy>>,
    mut events: EventReader<MyCollisionEvent>,
    mut score_events: EventWriter<ScoreEvent>,
    asset_server: Res<AssetServer>,
    sfx: Res<AudioChannel<SoundFX>>,
    attack: Res<Attack>,
) {
    for event in events.read() {
        if let MyCollisionEvent::WeaponHitsEnemy(event_entity) = event {
            let entity = commands.get_entity(*event_entity);
            if !entity.is_some() {
                continue;
            }
            let event_entity_id = entity.unwrap().id();
            for (entity, mut enemy, mut sprite) in &mut enemies {
                let entity_id = commands.get_entity(entity).unwrap().id();
                if entity_id == event_entity_id {
                    let health = enemy.health - (attack.damage_amount * attack.damage_scale) as f32;
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

fn collided_with_player(
    mut commands: Commands,
    mut collision_events: EventReader<EnemyHitPlayer>,
    mut enemies: Query<(Entity, &mut EnemySprite), With<Enemy>>
) {
    if enemies.is_empty() { return; }

    for collision_event in collision_events.read() {
        match enemies.get_mut(collision_event.0) {
            Err(_) => continue,
            Ok((_, mut enemy)) => {
                enemy.health -= 1.;
                if enemy.health <= 0. {
                    commands.get_entity(collision_event.0).unwrap().despawn();
                }
            }
        };
    }
}

pub fn cleanup_sprites(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
