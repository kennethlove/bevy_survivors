use crate::components::*;
use crate::constants::*;
use crate::{AppState, Scoreboard};
use bevy::prelude::*;

fn menu_button_system(
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

pub fn setup_hp(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: Query<(&Transform, &Pawn)>,
) {
    let mut location: &Transform = &Transform::from_translation(Vec3::new(0., 0., 0.));
    let mut pawn: &Pawn = &Pawn {
        health: 1.,
        speed: 1.,
    };

    if !player.is_empty() {
        let pl = player.single_mut();
        location = pl.0;
        pawn = pl.1;
    }

    let mut location = *location;
    location.translation.y += 20.;

    let font = asset_server.load("fonts/quaver.ttf");

    let text_style = TextStyle {
        color: Color::WHITE,
        font_size: 16.0,
        font,
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(pawn.health.to_string(), text_style.clone()),
            transform: location.clone(),
            ..default()
        },
        PlayerHealth,
    ));
}

pub fn update_hp(
    mut player: Query<(&Transform, &Pawn), Without<PlayerHealth>>,
    mut hp: Query<(&mut Text, &mut Transform), With<PlayerHealth>>,
) {
    if player.is_empty() {
        return;
    }

    let (location, pawn) = player.single_mut();
    let mut location = *location;
    location.translation.y += 20.;

    for (mut text, mut transform) in &mut hp {
        transform.translation = location.translation;
        text.sections[0].value = pawn.health.to_string();
    }
}

pub fn cleanup_hp(mut commands: Commands, query: Query<Entity, With<PlayerHealth>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Scoreboard>) {
    let font = asset_server.load("fonts/quaver.ttf");

    let text_style = TextStyle {
        color: Color::WHITE,
        font_size: 12.0,
        font,
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    margin: UiRect {
                        left: Val::Px(20.),
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
            parent.spawn((
                TextBundle::from_section(format!("Score {}", score.score), text_style.clone()),
                Score,
            ));
            parent.spawn((
                TextBundle::from_section(format!("Kills {}", score.kills), text_style.clone()),
                Kills,
            ));
        });
}

pub fn update_ui(
    mut params: ParamSet<(Query<&mut Text, With<Score>>, Query<&mut Text, With<Kills>>)>,
    scoreboard: Res<Scoreboard>,
) {
    for mut text in &mut params.p0() {
        text.sections[0].value = format!("Score: {}", scoreboard.score);
    }
    for mut text in &mut params.p1() {
        text.sections[0].value = format!("Kills: {}", scoreboard.kills);
    }
}

pub fn cleanup_ui(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction, &mut UiImage), With<Button>>,
) {
    for entity in &mut interaction_query.iter() {
        commands.entity(entity.0).despawn_recursive();
    }
}
