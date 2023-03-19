use std::collections::VecDeque;
use std::process::id;

use bevy::prelude::*;

use crate::{AnomalyMining, au_to_system, GalaxyCoordinate, RegisterTo, SimPosition, spawn_anom, spawn_inventory, spawn_station_at, UndockLoc};
use crate::map::GalaxyMap;

/// Return (target system ID, route to system)
pub fn get_system_in_range(
    range: usize,
    coord: &GalaxyCoordinate,
    routes: &Res<GalaxyMap>,
) -> Vec<(Entity, VecDeque<Entity>)> {
    let test = &routes.routes;
    let mut res: Vec<(Entity, VecDeque<Entity>)> =
        test.iter()
            .filter(|(a, b)| a.0 == coord.0 && b.len() <= range + 1)
            .map(|(a, b)| (a.1, b.clone()))
            .collect();

    res.sort_by(|a, b|
        a.1.len().cmp(&b.1.len())
    );

    println!("all routes : {:?}", res);

    return res;
}

pub fn spawn_station(commands : &mut Commands, coord: GalaxyCoordinate, pos: SimPosition) -> Entity{
    let id = commands.spawn((
        spawn_station_at(pos, coord.0),
        UndockLoc,
    )).id();
    commands.spawn(spawn_inventory(id));
    return id;
}

pub fn spawn_mining_anom(commands : &mut Commands, coord: GalaxyCoordinate, pos: SimPosition) -> Entity {
    let id = commands.spawn((
        spawn_anom(pos, coord.0),
        AnomalyMining { tracked: Vec::new() },
        RegisterTo(coord.0),
    )).id();

    return id;
}