use crate::{AnimationIndices, AnimationTimer, Pawn, Enemy};
use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;

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

#[derive(Bundle)]
pub struct KnightBundle {
    transform: Transform,
    sprite: SpriteSheetBundle,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
    pawn: Pawn,
}

impl Default for KnightBundle {
    fn default() -> Self {
        KnightBundle {
            transform: Transform::from_translation(STARTING_POSITION),
            sprite: SpriteSheetBundle::default(),
            animation_indices: IDLE_ANIMATION,
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            pawn: Pawn,
        }
    }
}

impl KnightBundle {
    pub fn setup_sprite(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let texture = asset_server.load("Warrior_Purple.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(192., 192.), 49, 1, None, None);
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
        mut enemy_query: Query<(&Transform, &Sprite), With<Enemy>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        let player = player_query.single_mut();
        for (enemy_transform, enemy_sprite) in &mut enemy_query.iter() {
            if keyboard_input.just_pressed(KeyCode::Space) {
                if player.0.translation.distance(enemy_transform.translation) < 100. {
                    info!("Player collided with enemy!");
                    info!("Player attacked enemy!");
                }
            }
        }
    }
}
