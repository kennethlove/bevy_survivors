use crate::{AnimationIndices, AnimationTimer, Enemy};
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

#[derive(Component)]
struct Torch;

#[derive(Bundle)]
pub struct GoblinBundle {
    pub transform: Transform,
    pub sprite: SpriteSheetBundle,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub pawn: Enemy,
}

impl Default for GoblinBundle {
    fn default() -> Self {
        GoblinBundle {
            transform: Transform::from_translation(Vec3::new(100., 100., 0.)),
            sprite: SpriteSheetBundle {
                ..default()
            },
            animation_indices: IDLE_ANIMATION,
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            pawn: Enemy,
        }
    }
}

impl GoblinBundle {
    pub fn setup_sprite(
        self: Self,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
        transform: Transform,
    ) {
        let texture = asset_server.load("Goblin_Yellow.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(192., 192.), 7, 5, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = IDLE_ANIMATION;

        commands.spawn((
            SpriteSheetBundle {
                texture,
                transform, // Controls the placement of the sprite
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Enemy,
        ));
    }
}
