use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;

const STARTING_POSITION: Vec3 = Vec3::ZERO;

const WEAPON_01: Weapon = Weapon {
    damage_amount: 5.,
    damage_frame_end: 4,
    damage_frame_start: 0,
    damage_scale: 1.,
};

#[derive(Component)]
pub struct Weapon {
    pub damage_amount: f32,
    pub damage_frame_end: usize,
    pub damage_frame_start: usize,
    pub damage_scale: f32,
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
                damage_amount: 0.,
                damage_frame_start: 0,
                damage_frame_end: 0,
                damage_scale: 1.,
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
        transform.translation.z = 0.;
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
            weapon: WEAPON_01,
        });
        commands.spawn((
            AudioBundle {
                source: asset_server.load("sfx/woosh2.ogg"),
                settings: PlaybackSettings::LOOP,
            },
            SFX,
        ));
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

    pub fn cleanup_sprite(mut commands: Commands, mut query: Query<Entity, With<Weapon>>) {
        for entity in &mut query {
            commands.entity(entity).despawn();
        }
    }
}
