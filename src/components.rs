use bevy::prelude::*;

#[derive(Component)]
pub struct Pawn {
    pub speed: f32,
    pub health: f32,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct TitleText;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct OptionsButton;

#[derive(Component)]
pub struct QuitButton;
