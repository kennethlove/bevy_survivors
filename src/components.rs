use bevy::prelude::*;

#[derive(Component)]
pub struct Pawn;

#[derive(Component)]
pub struct Enemy;

#[derive(Clone, Component, Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
