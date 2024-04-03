use crate::components::*;
use crate::constants::*;
use crate::AppState;

use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
};
pub struct CameraPlugin;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct UICamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, move_camera.run_if(in_state(AppState::InGame)));
    }
}
fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let canvas_size = Extent3d {
        width: WIDTH as u32,
        height: HEIGHT as u32,
        ..default()
    };

    // This Image serves as a canvas representing the UI layer
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    canvas.resize(canvas_size);
    let image_handle = images.add(canvas);

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        UICamera,
        UI_LAYER,
    ));

    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn move_camera(
    mut query: Query<(&mut Transform, &MainCamera), Without<Pawn>>,
    pawn_query: Query<&Transform, With<Pawn>>,
) {
    if pawn_query.iter().count() == 0 {
        return;
    }
    let mut camera = query.single_mut();
    camera.0.translation = pawn_query.single().translation.truncate().extend(10.0);
}
