use bevy::render::view::RenderLayers;

pub const DRAW_GIZMOS: bool = false;
pub const UI_LAYER: RenderLayers = RenderLayers::layer(9);

pub const WIDTH: f32 = 600.;
pub const HEIGHT: f32 = 400.;

pub const SPRITE_WIDTH: u32 = 16;
pub const SPRITE_HEIGHT: u32 = 16;

pub const PAWN_SPEED: f32 = 200.;
pub const PAWN_SPEED_FAST: f32 = 300.;
