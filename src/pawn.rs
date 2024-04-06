use crate::animation::{AnimationIndices, AnimationTimer};
use crate::collision::EnemyHitPlayer;
use crate::components::{Enemy, Pawn};
use crate::AppState;
use crate::constants::*;
use crate::{ScoreEvent, Scoreboard};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

const IDLE_ANIMATION: AnimationIndices = AnimationIndices { first: 0, last: 1 };
const RUN_ANIMATION: AnimationIndices = AnimationIndices { first: 1, last: 7 };
const STARTING_POSITION: Vec3 = Vec3::ZERO;

#[derive(Resource)]
pub struct Attack {
    pub damage_amount: f32,
    pub damage_scale: f32,
}

#[derive(Event)]
struct MovementEvent {
    movement: Option<Direction2d>,
}

#[derive(Component)]
enum Direction {
    Left,
    Right,
}

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Attack {
            damage_amount: 10.,
            damage_scale: 1.,
        })
        .init_state::<PawnState>()
        .add_plugins(InputManagerPlugin::<PawnAction>::default())
        .add_event::<MovementEvent>()
        .add_systems(OnEnter(AppState::InGame), spawn_pawn)
        .add_systems(OnExit(AppState::InGame), cleanup_pawn)
        .add_systems(
            Update,
            (pawn_movement, update_score, update_direction).run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            FixedUpdate,
            (update_pawn_direction, collide_enemies, move_pawn).run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Bundle)]
struct PawnBundle {
    sprite: SpriteSheetBundle,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
    pawn: Pawn,
    input_manager: InputManagerBundle<PawnAction>,
    direction: Direction,
}

impl Default for PawnBundle {
    fn default() -> Self {
        PawnBundle {
            sprite: SpriteSheetBundle {
                transform: Transform::from_translation(STARTING_POSITION),
                ..default()
            },
            animation_indices: IDLE_ANIMATION,
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            pawn: Pawn {
                speed: PAWN_SPEED,
                health: 1.,
            },
            input_manager: InputManagerBundle::with_map(PawnAction::default_input_map()),
            direction: Direction::Right,
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum PawnAction {
    // movement
    Idle,
    MoveUp,
    MoveRight,
    MoveDown,
    MoveLeft,
}

impl PawnAction {
    const DIRECTIONS: [Self; 4] = [
        PawnAction::MoveUp,
        PawnAction::MoveRight,
        PawnAction::MoveDown,
        PawnAction::MoveLeft,
    ];

    fn direction(self) -> Option<Direction2d> {
        match self {
            PawnAction::MoveUp => Some(Direction2d::Y),
            PawnAction::MoveRight => Some(Direction2d::X),
            PawnAction::MoveDown => Some(Direction2d::NEG_Y),
            PawnAction::MoveLeft => Some(Direction2d::NEG_X),
            _ => None,
        }
    }

    fn default_input_map() -> InputMap<PawnAction> {
        use PawnAction::*;
        let mut input_map = InputMap::default();

        input_map.insert(MoveUp, KeyCode::ArrowUp);
        input_map.insert(MoveUp, KeyCode::KeyW);
        input_map.insert(MoveUp, GamepadButtonType::DPadUp);

        input_map.insert(MoveRight, KeyCode::ArrowRight);
        input_map.insert(MoveRight, KeyCode::KeyD);
        input_map.insert(MoveRight, GamepadButtonType::DPadRight);

        input_map.insert(MoveDown, KeyCode::ArrowDown);
        input_map.insert(MoveDown, KeyCode::KeyS);
        input_map.insert(MoveDown, GamepadButtonType::DPadDown);

        input_map.insert(MoveLeft, KeyCode::ArrowLeft);
        input_map.insert(MoveLeft, KeyCode::KeyA);
        input_map.insert(MoveLeft, GamepadButtonType::DPadLeft);

        input_map
    }
}

fn spawn_pawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("pawns/purple_knight.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(16., 22.), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = IDLE_ANIMATION;

    let mut transform = Transform::from_translation(STARTING_POSITION);
    transform.translation.z = 9.;
    transform.scale = Vec3::splat(2.);

    commands.spawn((
        PawnBundle {
            sprite: SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                transform,
                ..default()
            },
            pawn: Pawn {
                speed: PAWN_SPEED,
                health: 100.,
            },
            ..default()
        },
        RigidBody::KinematicPositionBased,
        KinematicCharacterController {
            // apply_impulse_to_dynamic_bodies: true,
            ..default()
        },
        Collider::cuboid(8., 11.),
        PawnState::default(),
        ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
        ActiveEvents::COLLISION_EVENTS | ActiveEvents::CONTACT_FORCE_EVENTS,
        SolverGroups::new(PAWN_WEAPON_GROUP, Group::default()),
    ));
}

#[derive(Clone, Component, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum PawnState {
    #[default]
    Idle,
    Running,
}

fn move_pawn(
    mut query: Query<&mut KinematicCharacterController, With<Pawn>>,
    mut moves: EventReader<MovementEvent>,
    mut next_state: ResMut<NextState<PawnState>>,
    time: Res<Time>,
) {
    if query.is_empty() {
        return;
    }

    let mut pawn = query.single_mut();

    for event in moves.read() {
        let MovementEvent { movement } = event;
        if movement.is_some() {
            let direction = movement.unwrap();
            pawn.translation =
                Some(Vec2::new(direction.x, direction.y) * time.delta_seconds() * PAWN_SPEED);
            next_state.set(PawnState::Running);
        } else {
            next_state.set(PawnState::Idle);
        }
    }
}

fn pawn_movement(
    query: Query<&ActionState<PawnAction>, With<Pawn>>,
    mut event_writer: EventWriter<MovementEvent>,
) {
    if query.is_empty() {
        return;
    }

    let action_state = query.single();
    let mut direction_vector = Vec2::ZERO;

    for input_direction in PawnAction::DIRECTIONS {
        if action_state.pressed(&input_direction) {
            if let Some(direction) = input_direction.direction() {
                direction_vector += *direction;
            }
        }
    }

    let net_direction = Direction2d::new(direction_vector);

    if let Ok(direction) = net_direction {
        event_writer.send(MovementEvent {
            movement: Some(direction),
        });
    }
}

fn cleanup_pawn(mut commands: Commands, mut query: Query<Entity, With<Pawn>>) {
    for entity in &mut query {
        commands.entity(entity).despawn();
    }
}

fn collide_enemies(
    mut events: EventReader<EnemyHitPlayer>,
    mut player_query: Query<(&mut Pawn, &mut Sprite), Without<Enemy>>,
    mut state: ResMut<NextState<AppState>>,
) {
    let (mut player, mut sprite) = player_query.single_mut();
    let mut new_health = player.health.round() as isize;

    for _ in events.read() {
        new_health -= 1;
    }

    if new_health <= 0 {
        state.set(AppState::GameOver);
    } else {
        if new_health < player.health.round() as isize {
            sprite.color = Color::RED;
        } else {
            sprite.color = Color::WHITE;
        }
        player.health = new_health as f32;
    }
}

fn update_direction(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput)>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, controller) = query.single();

    if controller.desired_translation.x > 0. {
        commands.entity(entity).insert(Direction::Right);
    } else if controller.desired_translation.x < 0. {
        commands.entity(entity).insert(Direction::Left);
    }
}

fn update_pawn_direction(mut query: Query<(&mut Sprite, &Direction), With<Pawn>>) {
    if query.is_empty() {
        return;
    }

    let (mut sprite, direction) = query.single_mut();
    match direction {
        Direction::Right => sprite.flip_x = false,
        Direction::Left => sprite.flip_x = true,
    }
}

fn update_score(mut score: ResMut<Scoreboard>, mut events: EventReader<ScoreEvent>) {
    for event in events.read() {
        match event {
            ScoreEvent::Scored(amount) => {
                score.score += amount;
                score.kills += 1;
            }
            ScoreEvent::EnemyHit => {
                score.score += 10;
            }
        }
    }
}
