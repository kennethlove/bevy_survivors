use crate::components::*;
use bevy::prelude::*;

const STARTING_POSITION: Vec3 = Vec3::ZERO;

const WEAPON_01: Weapon = Weapon {
    damage_amount: 50.,
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

        commands
            .spawn(WeaponBundle {
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
            })
            .with_children(|parent| {
                parent.spawn((
                    AudioBundle {
                        source: asset_server.load("sfx/woosh2.ogg"),
                        settings: PlaybackSettings::LOOP,
                    },
                    SFX,
                ));
            });
    }

    pub fn move_weapon(
        mut query: Query<(&mut Transform, &Weapon), Without<Pawn>>,
        pawn_query: Query<&Transform, With<Pawn>>,
    ) {
        for mut transform in &mut query {
            let pawn_transform = pawn_query.single();
            transform.0.translation = pawn_transform.translation.truncate().extend(1.);
        }
    }

    pub fn cleanup_sprite(mut commands: Commands, mut query: Query<Entity, With<Weapon>>) {
        for entity in &mut query {
            commands.entity(entity).despawn();
        }
    }
}
