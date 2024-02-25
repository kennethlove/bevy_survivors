use crate::components::{AnimationIndices, AnimationTimer, Enemy, Pawn};
use crate::constants::*;
use bevy::input::keyboard::KeyCode;
use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::*;

const IDLE_ANIMATION: AnimationIndices = AnimationIndices { first: 0, last: 1 };
const RUN_ANIMATION: AnimationIndices = AnimationIndices { first: 1, last: 7 };
const STARTING_POSITION: Vec3 = Vec3::new(0., 0., 2.);

#[derive(Bundle)]
pub struct PawnBundle {
    transform: Transform,
    sprite: SpriteSheetBundle,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
    pawn: Pawn,
}

impl Default for PawnBundle {
    fn default() -> Self {
        PawnBundle {
            transform: Transform::from_translation(STARTING_POSITION),
            sprite: SpriteSheetBundle::default(),
            animation_indices: IDLE_ANIMATION,
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            pawn: Pawn,
        }
    }
}

impl PawnBundle {
    pub fn setup_sprite(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let texture = asset_server.load("16x32.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(16., 32.), 8, 64, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = IDLE_ANIMATION;

        commands.spawn((
            SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                transform: Transform::from_scale(Vec3::splat(2.)),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Pawn,
        ));
    }

    pub fn move_sprite(
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

                new_animation_indices = RUN_ANIMATION;
            }

            animation_indices.first = new_animation_indices.first;
            animation_indices.last = new_animation_indices.last;
            if atlas.index > animation_indices.last || atlas.index < animation_indices.first {
                atlas.index = animation_indices.first;
            }
        }
    }

    pub fn collisions(
        mut commands: Commands,
        mut player_query: Query<(&Transform, &Sprite), With<Pawn>>,
        enemy_query: Query<(Entity, &Transform), With<Enemy>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut gizmos: Gizmos,
    ) {
        let (player_transform, player_sprite) = player_query.single_mut();

        let mut sword_position = player_transform.translation.truncate() + Vec2::new(30., 0.);
        if player_sprite.flip_x {
            sword_position.x -= 115.;
        }

        let mut sword_pos = BoundingCircle::new(sword_position, 32.);
        sword_pos.center += Vec2::new(28., 0.);
        if DRAW_GIZMOS {
            gizmos.circle_2d(sword_pos.center, 32., Color::YELLOW);
        }

        for (enemy_entity, enemy_transform) in &mut enemy_query.iter() {
            let enemy_rect = Rect::from_center_size(
                enemy_transform.translation.truncate(),
                Vec2::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32),
            );
            let enemy_pos = Aabb2d::new(enemy_rect.center(), enemy_rect.size());

            if keyboard_input.pressed(KeyCode::Space) {
                let collision = sword_pos.intersects(&enemy_pos);
                if collision {
                    info!("Player attacked enemy!");
                    commands.entity(enemy_entity).despawn();
                }
            }
        }
    }
}
