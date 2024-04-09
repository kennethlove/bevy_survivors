use bevy::prelude::*;
use bevy_pkv::PkvStore;

#[derive(Resource)]
pub struct Settings {
    pub volume: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { volume: 1. }
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Settings::default())
            .add_systems(Startup, load_settings)
            .add_systems(Update, save_settings);
    }
}

fn load_settings(mut settings: ResMut<Settings>, pkv: Res<PkvStore>) {
    let volume: f32 = match pkv.get::<f32>("volume") {
        Ok(volume) => volume,
        Err(_) => 1.,
    };
    settings.volume = volume;
}

fn save_settings(settings: Res<Settings>, mut pkv: ResMut<PkvStore>) {
    pkv.set::<f32>("volume", &settings.volume).unwrap();
}
