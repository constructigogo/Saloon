use std::process::id;
use bevy::prelude::*;

use crate::{Destination, DVec3, m_to_system, Mass, SimPosition, ThrusterEngine};
use crate::space::galaxy::au_to_system;
use crate::space::ship;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct InitWarp;

#[derive(Component)]
pub struct Warping {
    speed: f64, // AU/s
}


pub fn check_for_warp_system(
    mut commands: Commands,
    mut query: Query<(Entity, &SimPosition, &Destination), (Or<(Added<Destination>, Changed<Destination>)>, Without<Warping>)>,
) {
    for (id, pos, desto) in query.iter() {
        match desto.0 {
            ship::DestoType::DPosition(target) => {
                let dist_left: f64 = (pos.0 - target.0).length();
                if dist_left>m_to_system(100000.0){
                    commands.entity(id).insert((InitWarp));
                }
            }
            ship::DestoType::TEntity(_) => {}
            ship::DestoType::None => {}
        }
        println!("toggle warp");
    }
}


pub fn init_warp_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Mass, &ThrusterEngine), With<InitWarp>>,
) {
    for (id, mass, thruster) in query.iter() {
        commands.entity(id).insert((Warping {
            speed: 0.5
        })).remove::<InitWarp>();
    }
}

pub fn warp_movement_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut SimPosition, &Destination, &Warping)>,
) {
    for (id, mut pos, desto, warping) in query.iter_mut() {
        match desto.0 {
            ship::DestoType::DPosition(target) => {
                let dir: DVec3 = (target.0 - pos.0).normalize();

                let dist_left: f64 = (pos.0 - target.0).length();
                let travel_dist: f64 = au_to_system(warping.speed) * time.delta_seconds_f64();

                let effective_travel: f64;

                if travel_dist > dist_left {
                    effective_travel = dist_left - m_to_system(350.0);
                    commands.entity(id).remove::<Warping>();
                } else {
                    effective_travel = travel_dist;
                }
                pos.0 += dir * effective_travel;
            }
            ship::DestoType::TEntity(_) => {}
            ship::DestoType::None => {}
        }
    }
}