use crate::components::{AnimationIndices, AnimationTimer, Enemy, Pawn};
use crate::constants::*;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
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
            sprite: SpriteSheetBundle { ..default() },
            animation_indices: IDLE_ANIMATION,
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            pawn: Enemy,
        }
    }
}

impl GoblinBundle {
    fn find_good_spot(
        goblins: Query<&Transform, With<Enemy>>,
        player: Query<&Transform, With<Pawn>>,
    ) -> Vec3 {
        let player_pos = player.single().translation;

        let mut x = fastrand::i32(..) % SAFE_WIDTH as i32;
        if fastrand::bool() {
            x = -x;
        }
        x = x.clamp(
            ((-SAFE_WIDTH + SPRITE_WIDTH as f32) / 2.) as i32,
            ((SAFE_WIDTH - SPRITE_WIDTH as f32) / 2.) as i32,
        );

        let mut y = fastrand::i32(..) % SAFE_HEIGHT as i32;
        if fastrand::bool() {
            y = -y;
        }
        y = y.clamp(
            ((-SAFE_HEIGHT + SPRITE_HEIGHT as f32) / 2.) as i32,
            ((SAFE_HEIGHT - SPRITE_HEIGHT as f32) / 2.) as i32,
        );

        let translation = Vec3::new(x as f32, y as f32, 0.);
        let sprite_area = Vec2::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32);
        if Aabb2d::new(translation.truncate(), sprite_area)
            .intersects(&Aabb2d::new(player_pos.truncate(), sprite_area))
        {
            return GoblinBundle::find_good_spot(goblins, player);
        }
        translation
    }

    pub fn spawn_goblins(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
        goblins: Query<&Transform, With<Enemy>>,
        player: Query<&Transform, With<Pawn>>,
    ) {
        let texture: Handle<Image> = asset_server.load("Goblin_Yellow.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(192., 192.), 7, 5, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = IDLE_ANIMATION;

        if goblins.iter().count() < 5 {
            let transform =
                Transform::from_translation(GoblinBundle::find_good_spot(goblins, player));

            commands.spawn((
                SpriteSheetBundle {
                    texture: texture,
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

    pub fn update_goblins(
        mut query: Query<&Transform, With<Enemy>>,
        mut gizmos: Gizmos,
    ) {
        for transform in &mut query {
            let image_size = Vec2::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32);
            let scaled = image_size * transform.scale.truncate();
            let bounding_box = Rect::from_center_size(transform.translation.truncate(), scaled);
            gizmos.rect_2d(bounding_box.center(), 0., bounding_box.size(), Color::WHITE);
        }
    }
}
