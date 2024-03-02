use crate::components::*;
use crate::constants::*;
use crate::AppState;
use bevy::prelude::*;

#[derive(Event)]
pub enum MenuEvent {
    Play,
    Options,
    Quit,
}

pub fn main_menu_button_system(
    mut state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(AppState::InGame);
    }

    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                if text.sections[0].value == "Play" {
                    state.set(AppState::InGame);
                } else if text.sections[0].value == "Options" {
                } else if text.sections[0].value == "Quit" {
                    std::process::exit(0);
                }
            }
            Interaction::Hovered => {
                text.sections[0].style.font_size = 26.0;
            }
            Interaction::None => {
                text.sections[0].style.font_size = 24.0;
            }
        }
    }
}

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/quaver.ttf");
    let texture_handle: Handle<Image> = asset_server.load("buttons/9slice.png");

    let text_style = TextStyle {
        color: Color::WHITE,
        font_size: 24.0,
        font,
    };

    let slicer = TextureSlicer {
        border: BorderRect::square(16.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.,
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        left: Val::Px(0.),
                        right: Val::Px(0.),
                        top: Val::Px(20.),
                        bottom: Val::Px(0.),
                    },
                    ..default()
                },
                ..default()
            },
            UI_LAYER,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Px(150.),
                            height: Val::Px(50.),
                            ..default()
                        },
                        image: texture_handle.clone().into(),
                        ..default()
                    },
                    ImageScaleMode::Sliced(slicer.clone()),
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play".to_string(),
                        text_style.clone(),
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Px(150.),
                            height: Val::Px(50.),
                            ..default()
                        },
                        image: texture_handle.clone().into(),
                        ..default()
                    },
                    ImageScaleMode::Sliced(slicer.clone()),
                    OptionsButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Options".to_string(),
                        text_style.clone(),
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Px(150.),
                            height: Val::Px(50.),
                            ..default()
                        },
                        image: texture_handle.clone().into(),
                        ..default()
                    },
                    ImageScaleMode::Sliced(slicer.clone()),
                    QuitButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit".to_string(),
                        text_style.clone(),
                    ));
                });
        });
}

pub fn cleanup_main_menu(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction, &mut UiImage), With<Button>>,
) {
    for entity in &mut interaction_query.iter() {
        commands.entity(entity.0).despawn_recursive();
    }
}
