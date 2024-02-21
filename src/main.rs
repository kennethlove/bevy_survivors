mod goblin;
mod knight;

use bevy::{
    prelude::*,
    window::{Window, WindowTheme},
};
use goblin::GoblinBundle;
use knight::{KnightBundle, KnightColor};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum AppState {
    #[default]
    Setup,
    Menu,
    InGame,
    Finished,
}

#[derive(Component)]
pub struct Pawn;

#[derive(Component)]
pub struct Enemy;

#[derive(Clone, Component, Debug)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
        .init_state::<AppState>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        prevent_default_event_handling: false,
                        resizable: false,
                        resolution: Vec2 { x: 800., y: 600. }.into(),
                        title: "Survivors".into(),
                        window_theme: Some(WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(
            Startup,
            (setup_camera, setup_background,),
        )
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(OnEnter(AppState::InGame), (setup_goblin, setup_player))
        .add_systems(Update, animate_sprites)
        // .add_systems(
        //     FixedUpdate,
        //     (KnightBundle::move_sprite, KnightBundle::collisions),
        // )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("buttons/Button_Blue_9Slides.png");

    let slicer = TextureSlicer {
        border: BorderRect::square(22.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.,
    };

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        let colors = [KnightColor::Blue, KnightColor::Purple, KnightColor::Red, KnightColor::Yellow];
        for color in colors {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.),
                            height: Val::Px(50.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(20.)),
                            ..default()
                        },
                        image: image.clone().into(),
                        ..default()
                    },
                    ImageScaleMode::Sliced(slicer.clone()),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(format!("{:?}", color), TextStyle {
                        font_size: 40.,
                        color: Color::DARK_GRAY,
                        ..default()
                    },
                ));
            });
        }
    });
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>, mut next_state: ResMut<NextState<AppState>>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("floors/floor_1.png"),
            transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
            sprite: Sprite {
                custom_size: Some(Vec2::from((800., 600.))),
                ..default()
            },
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 2.,
        },
    ));

    next_state.set(AppState::Menu);
}

fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn setup_player(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    KnightBundle::default().setup_sprite(commands, asset_server, texture_atlas_layouts);
}

fn setup_goblin(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let transform = Transform::from_translation(Vec3::new(100., 100., 0.));
    GoblinBundle::default().setup_sprite(commands, asset_server, texture_atlas_layouts, transform);
}
