use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

use crate::{around_pos, au_to_system, Destination, DestoType, DisplayableGalaxyEntityBundle, DVec3, GalaxyCoordinate, GalaxyEntityBundle, GalaxyGateTag, GalaxySpawnGateBundle, GateDestination, m_to_system, SimPosition, SolarSystem};
use crate::map::TravelRoute;
use crate::route::ContinueRoute;

#[derive(Component)]
pub struct TakeGate {
    pub(crate) from: Entity,
    pub(crate) to: Entity,
    pub(crate) spool_up: Timer,
}


#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct RegisterGateToSystem {
    coord: Entity,
    gate_to: Entity,
}

pub(crate) fn init_make_portal(
    mut commands: &mut Commands,
    pos_list: &HashMap<Entity, DVec3>,
    a: Entity,
    b: Entity,
) {
    let dir: DVec3 = (*pos_list.get(&b).unwrap() - *pos_list.get(&a).unwrap()).normalize();
    let gA = commands.spawn((
        GalaxySpawnGateBundle {
            tag: GalaxyGateTag,
            disp: DisplayableGalaxyEntityBundle {
                display: SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1.0, 1.0, 1.0),
                        custom_size: Some(Vec2::new(8.0, 8.0)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::ZERO,
                        ..default()
                    },
                    visibility: Visibility { is_visible: true },
                    ..default()
                },
                galaxy: GalaxyEntityBundle {
                    galaxy_coord: GalaxyCoordinate(a),
                    simulation_position: SimPosition(dir * au_to_system(10.0)),
                },
            },
        },
    )
    ).id();

    let gB = commands.spawn((
        GalaxySpawnGateBundle {
            tag: GalaxyGateTag,
            disp: DisplayableGalaxyEntityBundle {
                display: SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1.0, 1.0, 1.0),
                        custom_size: Some(Vec2::new(8.0, 8.0)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::ZERO,
                        ..default()
                    },
                    visibility: Visibility { is_visible: true },
                    ..default()
                },
                galaxy: GalaxyEntityBundle {
                    galaxy_coord: GalaxyCoordinate(b),
                    simulation_position: SimPosition(-dir * au_to_system(10.0)),
                },
            },
        },
    )).id();

    commands.entity(gA).insert((
        GateDestination(b),
        RegisterGateToSystem {
            coord: a,
            gate_to: gB,
        },
    ));
    commands.entity(gB).insert((
        GateDestination(a),
        RegisterGateToSystem {
            coord: b,
            gate_to: gA,
        },
    ));
}

pub fn register_gate_system(
    mut commands: Commands,
    mut systems: Query<(&mut SolarSystem)>,
    query: Query<(Entity, &GateDestination, &RegisterGateToSystem), (Added<RegisterGateToSystem>)>,
) {
    for (id, desto, order) in query.iter() {
        let mut sys = systems.get_mut(order.coord).unwrap();
        sys.gates.insert(desto.0, (id, order.gate_to));
        commands.entity(id).remove::<RegisterGateToSystem>();
    }
}

pub fn take_gate_added_system(
    mut commands: Commands,
    gate: Query<(&SimPosition)>,
    query: Query<(Entity, &GalaxyCoordinate, &SimPosition, &TravelRoute, &TakeGate), Added<TakeGate>>,
) {
    for (id, coord, pos, route, order) in &query {
        let (t_pos) = gate.get(order.from).unwrap();
        let dist = (pos.0 - t_pos.0).length();

        if dist > m_to_system(50.0) {
            commands.entity(id).insert((
                Destination(DestoType::DPosition(around_pos(*t_pos, 25.0)))
            ));
        }
    }
}

pub fn take_gate_system(
    mut commands: Commands,
    time: Res<Time>,
    gate: Query<(&GalaxyCoordinate, &SimPosition)>,
    mut query: Query<(Entity, &GalaxyCoordinate, &SimPosition, &TravelRoute, &mut TakeGate)>,
) {
    for (id, coord, pos, route, mut order) in &mut query {
        let (f_coord, f_pos) = gate.get(order.from).unwrap();
        let dist = (pos.0 - f_pos.0).length();
        if dist <= m_to_system(50.0) {
            if order.spool_up.finished() {
                let (t_coord, t_pos) = gate.get(order.to).unwrap();
                commands.entity(id)
                    .insert((
                        GalaxyCoordinate(t_coord.0),
                        around_pos(*t_pos, 250.0),
                    ))
                    .remove::<TakeGate>();

                if route.route.len() > 0 {
                    commands.entity(id).insert((ContinueRoute));
                    println!("still has to travel to : {:?}", route.route);
                } else {
                    println!("route finished");
                    commands.entity(id).remove::<TravelRoute>();
                }
            }
            else {
                order.spool_up.tick(Duration::from_secs(1));
            }
        }
    }
}