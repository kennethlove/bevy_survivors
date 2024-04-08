use crate::AppState;
use bevy::prelude::*;
use bevy_kira_audio::{
    Audio, AudioApp, AudioChannel, AudioControl, AudioEasing, AudioInstance, AudioTween,
};
use std::time::Duration;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_channel::<BackgroundMusic>()
            .add_audio_channel::<SoundFX>()
            .add_plugins(bevy_kira_audio::prelude::AudioPlugin)
            .add_systems(Startup, setup_music)
            .add_systems(Update, (volume_control, pause_audio));
    }
}

#[derive(Resource)]
struct AudioHandle(Handle<AudioInstance>);

#[derive(Resource)]
pub struct BackgroundMusic;

#[derive(Resource)]
pub struct SoundFX;

fn pause_audio(
    state: Res<State<AppState>>,
    mut bg: ResMut<Assets<AudioInstance>>,
    sfx: ResMut<AudioChannel<SoundFX>>,
    handle: Res<AudioHandle>,
) {
    if let Some(instance) = bg.get_mut(&handle.0) {
        match state.get() {
            AppState::InGame => {
                instance.set_volume(
                    0.5,
                    AudioTween::new(Duration::from_secs(1), AudioEasing::Linear),
                );
                sfx.resume()
                    .fade_in(AudioTween::new(Duration::from_secs(1), AudioEasing::Linear));
            }
            _ => {
                instance.set_volume(
                    0.1,
                    AudioTween::new(Duration::from_secs(1), AudioEasing::Linear),
                );
                sfx.pause()
                    .fade_out(AudioTween::new(Duration::from_secs(1), AudioEasing::Linear));
            }
        }
    }
}

fn setup_music(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let handle = audio
        .play(asset_server.load("music/Arcade.ogg"))
        .with_volume(0.5)
        .looped()
        .fade_in(AudioTween::new(Duration::from_secs(1), AudioEasing::Linear))
        .handle();
    commands.insert_resource(AudioHandle(handle));
}

fn volume_control(
    input: Res<ButtonInput<KeyCode>>,
    background: Res<AudioHandle>) {

    mut bg: ResMut<Assets<AudioInstance>>,
    sfx: ResMut<AudioChannel<SoundFX>>,
    handle: Res<AudioHandle>,
    let vol = background.get_volume();
    if input.just_pressed(KeyCode::Equal) {
        background.set_volume(vol += 0.1);
    } else if input.just_pressed(KeyCode::Minus) {
        background.set_volume(vol -= 0.1);
    }
}
