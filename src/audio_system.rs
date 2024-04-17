use crate::settings::Settings;
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

fn setup_music(mut commands: Commands, asset_server: Res<AssetServer>, settings: Res<Settings>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("music/Arcade.ogg"),
            settings: PlaybackSettings {
                volume: Volume::new(settings.volume),
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
    mut settings: ResMut<Settings>,
) {
    if let Ok(sink) = bgm.get_single() {
        let mut volume = sink.volume();

        if keyboard_input.just_pressed(KeyCode::Equal) {
            volume += 0.1;
        } else if keyboard_input.just_pressed(KeyCode::Minus) {
            volume -= 0.1;
        } else if keyboard_input.just_pressed(KeyCode::Digit0) {
            volume = 0.;
        }
        volume = volume.clamp(0., 1.);
        sink.set_volume(volume);
        settings.volume = volume;
    }
}
