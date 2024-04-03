use bevy::{
    asset::AssetMetaCheck,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowTheme,
};
use bevy_pkv::PkvStore;

use bevy_survivors::{
    animation::AnimationPlugin, audio::AudioPlugin, background::BackgroundPlugin,
    camera::CameraPlugin, components::*, constants::*, enemy::EnemyPlugin, menu::*,
    pawn::PawnPlugin, ui::*, weapon::WeaponPlugin, AppState, CollisionEvent, ScoreEvent,
    Scoreboard,
};

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Scoreboard { score: 0, kills: 0 })
        .insert_resource(PkvStore::new("kennethlove", "Survivors"))
        .init_state::<AppState>()
        .add_event::<ScoreEvent>()
        .add_event::<MenuEvent>()
        .add_event::<CollisionEvent>()
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
            AnimationPlugin,
            AudioPlugin,
            BackgroundPlugin,
            CameraPlugin,
            EnemyPlugin,
            PawnPlugin,
            WeaponPlugin,
        ))
        .add_systems(OnEnter(AppState::MainMenu), (setup_title, setup_main_menu))
        .add_systems(
            OnExit(AppState::MainMenu),
            (cleanup_title, cleanup_main_menu),
        )
        .add_systems(OnExit(AppState::InGame), cleanup_ui)
        .add_systems(OnEnter(AppState::InGame), (setup_ui, setup_hp))
        .add_systems(OnExit(AppState::InGame), cleanup_hp)
        .add_systems(OnEnter(AppState::GameOver), setup_game_over)
        .add_systems(OnExit(AppState::GameOver), (cleanup_game_over, reset))
        .add_systems(
            Update,
            (
                main_menu_button_system,
                (update_ui, update_hp, pause, game_over_button_system)
                    .run_if(in_state(AppState::InGame)),
            ),
        )
        // .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    let title_font: Handle<Font> = asset_server.load("fonts/DungeonFont.ttf");
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                top: Val::Px(-100.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Survivors".to_string(),
                    TextStyle {
                        font_size: 100.0,
                        color: Color::WHITE,
                        font: title_font,
                    },
                )
                .with_text_justify(JustifyText::Center),
                UI_LAYER,
                TitleText,
            ));
        });
}

fn cleanup_title(mut commands: Commands, query: Query<Entity, With<Text>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
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
