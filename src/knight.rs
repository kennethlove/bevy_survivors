use crate::{AnimationIndices, AnimationTimer, Enemy, Pawn};
use bevy::input::keyboard::KeyCode;
use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

const IDLE_ANIMATION: AnimationIndices = AnimationIndices { first: 1, last: 5 };
const RUN_ANIMATION: AnimationIndices = AnimationIndices { first: 6, last: 12 };
const ATTACK_ANIMATION: AnimationIndices = AnimationIndices {
    first: 13,
    last: 15,
};
const HEAVY_ATTACK_ANIMATION: AnimationIndices = AnimationIndices {
    first: 16,
    last: 22,
};
const STARTING_POSITION: Vec3 = Vec3::new(0., 0., 2.);

#[derive(Component)]
pub enum KnightColor {
    Yellow,
    Blue,
    Purple,
    Red,
}

#[derive(Bundle)]
pub struct KnightBundle {
    transform: Transform,
    sprite: SpriteSheetBundle,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
    pawn: Pawn,
    color: KnightColor,
}

impl Default for KnightBundle {
    fn default() -> Self {
        KnightBundle {
            transform: Transform::from_translation(STARTING_POSITION),
            sprite: SpriteSheetBundle::default(),
            animation_indices: IDLE_ANIMATION,
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            pawn: Pawn,
            color: KnightColor::Red,
        }
    }
}

impl KnightBundle {
    pub fn setup_sprite(
        self: Self,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let filename = match self.color {
            KnightColor::Yellow => "Warrior_Yellow.png",
            KnightColor::Blue => "Warrior_Blue.png",
            KnightColor::Purple => "Warrior_Purple.png",
            KnightColor::Red => "Warrior_Red.png",
        };
        let texture = asset_server.load(filename);
        let layout = TextureAtlasLayout::from_grid(Vec2::new(192., 192.), 6, 8, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = IDLE_ANIMATION;

        commands.spawn((
            SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Pawn,
            self.color,
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
                transform.translation += direction.normalize() * 100. * time.delta_seconds();
                new_animation_indices = RUN_ANIMATION;
            }

            if keyboard_input.pressed(KeyCode::Space) {
                new_animation_indices = ATTACK_ANIMATION;
            }
            if keyboard_input.pressed(KeyCode::ControlLeft)
                || keyboard_input.pressed(KeyCode::ControlRight)
            {
                new_animation_indices = HEAVY_ATTACK_ANIMATION;
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
        mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
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
        gizmos.circle_2d(sword_pos.center, 32., Color::YELLOW);

        for (enemy_entity, enemy_transform) in &mut enemy_query.iter() {
            let enemy_pos = Aabb2d::new(enemy_transform.translation.truncate(), Vec2::new(32., 32.));
            gizmos.rect_2d(enemy_pos.center(), 0., Vec2::new(64., 64.), Color::BLUE);

            if keyboard_input.pressed(KeyCode::Space) {
                let collision = sword_pos.intersects(&enemy_pos);
                if collision {
                    info!("Player collided with enemy!");
                    info!("Player attacked enemy!");
                    commands.entity(enemy_entity).despawn();
                }
            }
        }
    }
}
