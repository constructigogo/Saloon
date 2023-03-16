use std::collections::VecDeque;

use bevy::prelude::*;

use crate::GalaxyCoordinate;
use crate::map::GalaxyMap;

/// Return (target system ID, route to system)
pub fn get_system_in_range(
    range: usize,
    coord: &GalaxyCoordinate,
    routes: &Res<GalaxyMap>,
) -> Vec<(Entity, VecDeque<Entity>)> {
    let test = &routes.routes;
    let mut res : Vec<(Entity, VecDeque<Entity>)> =
        test.iter()
            .filter(|(a, b)| a.0 == coord.0 && b.len() <= range + 1)
            .map(|(a, b)| (a.1, b.clone()))
            .collect();

    res.sort_by(|a,b|
        a.1.len().cmp(&b.1.len())
    );

    println!("all routes : {:?}",res);

    return res;
}