use bevy::prelude::*;

#[derive(Component)]
pub struct Pawn {
    pub speed: f32,
    pub health: f32,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Clone, Component, Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct UICamera;

#[derive(Component)]
pub struct TitleText;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct OptionsButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct Score;

#[derive(Component)]
pub struct Kills;

#[derive(Component)]
pub struct PlayerHealth;

#[derive(Component)]
pub struct BackgroundMusic;

#[derive(Component)]
pub struct SFX;
