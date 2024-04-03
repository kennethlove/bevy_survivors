use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod components;
pub mod constants;
pub mod enemy;
pub mod menu;
pub mod pawn;
pub mod ui;
pub mod weapon;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum AppState {
    #[default]
    Setup,
    MainMenu,
    OptionMenu,
    InGame,
    GameOver,
}

#[derive(Resource)]
pub struct Scoreboard {
    pub score: u32,
    pub kills: u32,
}

#[derive(Resource)]
pub struct BackgroundMusic;

#[derive(Resource)]
pub struct SoundFX;

#[derive(Serialize, Deserialize, Debug)]
pub struct HighScore {
    pub score: u32,
    pub kills: u32,
}

#[derive(Event)]
pub enum ScoreEvent {
    Scored(u32),
    EnemyHit,
}

#[derive(Event)]
pub enum MovementEvent {
    Move(Vec2),
}

#[derive(Event)]
pub enum CollisionEvent {
    WeaponHitsEnemy(Entity),
    EnemyHitsPawn(Entity),
}
