use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{components::Pawn, utils::display_events};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyHitPlayer>()
            .add_event::<EnemyHitWeapon>()
            .add_systems(Update, enemy_collide_player);
    }
}

#[derive(Event, Debug)]
pub struct EnemyHitPlayer(pub Entity);

#[derive(Event)]
pub struct EnemyHitWeapon;

// Enemies collide with player
fn enemy_collide_player(
    mut collision_events: EventReader<CollisionEvent>,
    player: Query<Entity, With<Pawn>>,
    mut enemy_hit_player: EventWriter<EnemyHitPlayer>,
) {
    if player.is_empty() { return }

    let player = player.single();

    for event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            if player == *entity1 {
                enemy_hit_player.send(EnemyHitPlayer(*entity2));
            }
        }
    }
}


// Enemies collide with weapon
