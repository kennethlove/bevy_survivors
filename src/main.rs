use bevy::{
    asset::AssetMetaCheck,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowTheme,
};
use bevy_pkv::PkvStore;
use bevy_rapier2d::prelude::*;

use bevy_survivors::constants::*;
use bevy_survivors::{
    animation::AnimationPlugin, audio_system::AudioPlugin, background::BackgroundPlugin,
    camera::CameraPlugin, collision::CollisionPlugin, enemy::EnemyPlugin, menu::MenuPlugin,
    pawn::PawnPlugin, settings::SettingsPlugin, ui::UIPlugin, weapon::WeaponPlugin,
};
use bevy_survivors::{AppState, MyCollisionEvent, ScoreEvent, Scoreboard};

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Scoreboard { score: 0, kills: 0 })
        .insert_resource(PkvStore::new("kennethlove", "Survivors"))
        .init_state::<AppState>()
        .add_event::<ScoreEvent>()
        .add_event::<MyCollisionEvent>()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        prevent_default_event_handling: false,
                        resizable: false,
                        resolution: Vec2 {
                            x: WIDTH,
                            y: HEIGHT,
                        }
                        .into(),
                        title: "Survivors".into(),
                        window_theme: Some(WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            // FrameTimeDiagnosticsPlugin,
            // LogDiagnosticsPlugin::default(),
        ))
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins((
            AnimationPlugin,
            AudioPlugin,
            BackgroundPlugin,
            CameraPlugin,
            CollisionPlugin,
            EnemyPlugin,
            MenuPlugin,
            PawnPlugin,
            SettingsPlugin,
            WeaponPlugin,
            UIPlugin,
        ))
        .add_systems(OnExit(AppState::GameOver), reset)
        .add_systems(Update, pause.run_if(in_state(AppState::InGame)))
        // .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn pause(mut state: ResMut<NextState<AppState>>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        state.set(AppState::MainMenu);
    }
}

fn reset(mut scoreboard: ResMut<Scoreboard>) {
    scoreboard.score = 0;
    scoreboard.kills = 0;
}
