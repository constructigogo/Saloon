use std::time::Duration;
use bevy::prelude::*;
use crate::{around_pos, Destination, DestoType, GalaxyCoordinate, GalaxyGateTag, m_to_system, SimPosition, SolarSystem};
use crate::gates::TakeGate;
use crate::map::{GalaxyMap, TravelRoute, TravelTo};
use crate::warp::Warping;


#[derive(Component)]
pub struct ContinueRoute;

pub fn continue_route_system(
    mut commands: Commands,
    system: Query<(&SolarSystem)>,
    gate: Query<(&SimPosition), With<GalaxyGateTag>>,
    query: Query<(Entity, &GalaxyCoordinate, &SimPosition, &TravelRoute), With<ContinueRoute>>,
) {
    for (id, coord, pos, route) in query.iter() {
        let (gate_from, _) = system.get(coord.0).unwrap().gates.get(&route.route[0]).unwrap();
        let gate_pos = gate.get(*gate_from).unwrap();

        commands.entity(id).insert((
            Destination(DestoType::DPosition((around_pos(*gate_pos,500.0))))
        )).remove::<ContinueRoute>();
    }
}

pub fn travel_route_system(
    mut commands: Commands,
    system: Query<(&SolarSystem)>,
    gate: Query<(&SimPosition), With<GalaxyGateTag>>,
    mut query: Query<(Entity, &GalaxyCoordinate, &SimPosition, &Destination, &mut TravelRoute), (Without<Warping>, Without<TakeGate>)>,
) {
    for (id, coord, pos, dest, mut route) in &mut query {
        match dest.0 {
            DestoType::None => {}
            DestoType::TEntity(_) => {}
            DestoType::DPosition(target) => {
                let dist: f64 = (pos.0 - target.0).length();
                if dist <= m_to_system(10.0) {
                    println!("{:?}",dist/m_to_system(1.0));
                    let travel_dir = route.route.pop_front().unwrap();
                    let (gate_from_id, gate_to_id) = system.get(coord.0).expect("system?").gates.get(&travel_dir).unwrap();
                    commands.entity(id).insert(
                        (
                            TakeGate {
                                from: *gate_from_id,
                                to: *gate_to_id,
                                spool_up:Timer::from_seconds(1.0,TimerMode::Once),
                            }
                        ));
                }
            }
        }
    }
}

pub fn on_travel_added(
    mut commands: Commands,
    map: Res<GalaxyMap>,
    system: Query<(&SolarSystem)>,
    gate: Query<(&SimPosition, &GalaxyGateTag)>,
    mut query: Query<(Entity, &GalaxyCoordinate, &mut Destination, &TravelTo)>,
) {
    for (id, coord, mut desto, to) in query.iter_mut() {
        let route = map.routes.get(&(coord.0, to.0));
        match route {
            None => {}
            Some(val) => {
                println!("move to");
                println!("gates : {:?}, looking for {:?}", system.get(coord.0).expect("system???").gates, val[1]);
                let (gate_from, gate_to) = system.get(coord.0).expect("system???").gates.get(&val[1]).expect("gate???????");

                let mut copy_route = val.clone();
                copy_route.pop_front();

                commands.entity(id).insert(
                    (
                        TravelRoute { route: copy_route },
                        Destination(
                            DestoType::DPosition((around_pos(*gate.get(*gate_from).unwrap().0,500.0)))
                        )
                    )).remove::<TravelTo>();
            }
        }
    }
}
