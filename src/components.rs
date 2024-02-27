use bevy::prelude::*;

#[derive(Component)]
pub struct Pawn;

#[derive(Component)]
pub struct Enemy {
    pub health: u32,
}

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
pub struct Canvas;

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
