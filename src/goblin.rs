use crate::components::{AnimationIndices, AnimationTimer, Enemy, Pawn};
use crate::constants::*;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::{animation, prelude::*};

const IDLE_ANIMATION: AnimationIndices = AnimationIndices {
    first: 112,
    last: 113,
};
const RUN_ANIMATION: AnimationIndices = AnimationIndices {
    first: 112,
    last: 119,
};

#[derive(Component)]
struct Torch;

#[derive(Bundle)]
pub struct GoblinBundle {
    // pub transform: Transform,
    pub sprite: SpriteSheetBundle,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub pawn: Enemy,
}

impl Default for GoblinBundle {
    fn default() -> Self {
        GoblinBundle {
            sprite: SpriteSheetBundle::default(),
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
        let texture: Handle<Image> = asset_server.load("16x32.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(16., 32.), 8, 64, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = IDLE_ANIMATION;

        if goblins.iter().count() < 3 {
            let mut transform =
                Transform::from_translation(GoblinBundle::find_good_spot(goblins, player));
            transform = transform.with_scale(Vec3::splat(2.));

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
        mut params: ParamSet<(
            Query<&Transform, With<Pawn>>,
            Query<
                (
                    &mut Transform,
                    &mut AnimationIndices,
                    &mut TextureAtlas,
                    &mut Sprite,
                ),
                With<Enemy>,
            >,
        )>,
        mut gizmos: Gizmos,
    ) {
        if DRAW_GIZMOS {
            for (transform, _, _, _) in &mut params.p1() {
                let image_size = Vec2::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32);
                let scaled = image_size * transform.scale.truncate();
                let bounding_box = Rect::from_center_size(transform.translation.truncate(), scaled);
                gizmos.rect_2d(bounding_box.center(), 0., bounding_box.size(), Color::WHITE);
            }
        }

        let player_pos = params.p0().single().translation;
        for (mut transform, mut animation_indices, mut atlas, mut sprite) in &mut params.p1() {
            let mut direction = player_pos - transform.translation;
            direction = direction.normalize();
            sprite.flip_x = direction.x < 0.;
            transform.translation += direction * GOBLIN_SPEED;
            let mut new_animation_indices = IDLE_ANIMATION.clone();
            new_animation_indices.first = RUN_ANIMATION.first;
            new_animation_indices.last = RUN_ANIMATION.last;

            if player_pos.distance(transform.translation) < 50. {
                info!("Enemy attacks player");
            }

            animation_indices.first = new_animation_indices.first;
            animation_indices.last = new_animation_indices.last;
            if atlas.index > animation_indices.last || atlas.index < animation_indices.first {
                atlas.index = animation_indices.first;
            }
        }
    }
}
