use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_kira_audio::prelude::AudioPlugin)
            .add_systems(Startup, setup_music)
            .add_systems(Update, volume_control);
    }
}

#[derive(Component)]
struct BGMusic;

fn setup_music(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("music/Arcade.ogg"),
            settings: PlaybackSettings {
                volume: Volume::new(0.5),
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        BGMusic,
    ));
}

fn volume_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    bgm: Query<&AudioSink, With<BGMusic>>,
) {
    if let Ok(sink) = bgm.get_single() {
        if keyboard_input.just_pressed(KeyCode::Equal) {
            sink.set_volume(sink.volume() + 0.1);
        } else if keyboard_input.just_pressed(KeyCode::Minus) {
            sink.set_volume(sink.volume() - 0.1);
        } else if keyboard_input.just_pressed(KeyCode::Digit0) {
            sink.set_volume(0.0);
        }
    }
}
