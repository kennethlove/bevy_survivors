use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{components::Pawn, weapon::Weapon};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyHitPlayer>()
            .add_event::<EnemyHitWeapon>()
            .add_systems(Update, (enemy_collide_player, enemy_collide_weapon));
    }
}

#[derive(Event, Debug)]
pub struct EnemyHitPlayer(pub Entity);

#[derive(Event)]
pub struct EnemyHitWeapon(pub Entity);

// Enemies collide with player
fn enemy_collide_player(
    mut collision_events: EventReader<CollisionEvent>,
    player: Query<Entity, With<Pawn>>,
    mut enemy_hit_player: EventWriter<EnemyHitPlayer>,
) {
    if player.is_empty() {
        return;
    }

    let player = player.single();

    for event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            if player == *entity1 {
                enemy_hit_player.send(EnemyHitPlayer(*entity2));
            }
        }
    }
}

#[derive(Component)]
pub struct Collided;

// Enemies collide with weapon
fn enemy_collide_weapon(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    weapon: Query<Entity, With<Weapon>>,
    mut enemy_hit_player: EventWriter<EnemyHitWeapon>,
) {
    if weapon.is_empty() {
        return;
    }

    let weapon = weapon.single();

    for event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            if weapon == *entity1 {
                commands.entity(*entity2).insert(Collided);
                enemy_hit_player.send(EnemyHitWeapon(*entity2));
            }
        } else if let CollisionEvent::Stopped(entity1, entity2, _) = event {
            if weapon == *entity1 {
                if let Some(mut entity_commands) = commands.get_entity(*entity2) {
                    entity_commands.remove::<Collided>();
                }
            }
        }
    }
}
