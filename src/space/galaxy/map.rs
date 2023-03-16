use std::collections::VecDeque;
use std::process::id;
use std::thread::spawn;

use bevy::prelude::*;
use bevy::utils::hashbrown::*;
use bevy_mod_picking::{PickableBundle, Selection};
use bevy_prototype_debug_lines::DebugLines;
use big_brain::prelude::*;

use crate::{AnomalyMining, around_pos, au_to_system, DepositOre, Destination, DestoType, DisplayableGalaxyEntityBundle, DVec3, GalaxyCoordinate, GalaxyEntityBundle, GalaxyGateTag, GalaxySpawnGateBundle, GateDestination, m_to_system, MaterialMesh2dBundle, Mine, MineAnom, MoveToAnom, RegisterTo, Rendered, SimPosition, SolarSystem, spawn_anom, spawn_inventory, spawn_station_at, SystemMap, UndockingFrom, UndockLoc, ViewState, Weapon, WeaponBundle, WeaponConfig, WeaponInRange, WeaponSize, WeaponTarget, WeaponType};
use crate::AI::utils::get_system_in_range;
use crate::space::pilot::spawn_new_pilot;
use crate::ViewState::GALAXY;
use crate::warp::Warping;

#[derive(Component, Deref)]
pub struct TravelRoute {
    pub route: VecDeque<Entity>,
}

#[derive(Component, Deref)]
pub struct TravelTo(pub Entity);

#[derive(Resource)]
pub struct GalaxyMap {
    pub routes: HashMap<(Entity, Entity), VecDeque<Entity>>,
}


pub fn generate_map_pathfinding_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cluster: ResMut<SystemMap>,
    mut map: ResMut<GalaxyMap>,
) {
    let positions: Vec<DVec3> = vec![
        DVec3::new(-150.0, -75.0, 0.0) * m_to_system(10.0),
        DVec3::new(0.0, 75.0, 0.0) * m_to_system(10.0),
        DVec3::new(150.0, -75.0, 0.0) * m_to_system(10.0),
        DVec3::new(150.0, 75.0, 0.0) * m_to_system(10.0),
        DVec3::new(0.0, -75.0, 0.0) * m_to_system(10.0),
    ];

    let mut adj: Vec<(Entity, Vec<usize>)> = Vec::new();
    let mut adj_hash: HashMap<Entity, Vec<Entity>> = HashMap::new();
    let mut pos_hash: HashMap<Entity, DVec3> = HashMap::new();

    for pos in positions.iter() {
        let id = commands.spawn(
            (
                SolarSystem {
                    anomalies: Vec::new(),
                    gates: HashMap::new(),
                },
                UndockLoc,
                SimPosition(*pos),
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform {
                        translation: Vec3::ZERO,
                        scale: Vec3 { x: 64.0, y: 64.0, z: 1.0 },
                        ..default()
                    },
                    visibility: Visibility { is_visible: true },
                    ..default()
                },
                PickableBundle::default(),
                Rendered,
            )).remove::<Selection>().id();

        adj.push((id, Vec::new()));
        adj_hash.insert(id, Vec::new());
        pos_hash.insert(id, *pos);

        cluster.0.push(id);
    }

    add_edge(&mut adj, 0, 1);
    add_edge(&mut adj, 1, 2);
    add_edge(&mut adj, 3, 1);
    add_edge(&mut adj, 3, 2);
    add_edge(&mut adj, 2, 4);
    //add_edge(&mut adj,2,0);
    add_edge_hash(&mut adj_hash, adj[0].0, adj[1].0);
    init_make_portal(&mut commands, &pos_hash, adj[0].0, adj[1].0);
    add_edge_hash(&mut adj_hash, adj[1].0, adj[2].0);
    init_make_portal(&mut commands, &pos_hash, adj[1].0, adj[2].0);
    add_edge_hash(&mut adj_hash, adj[3].0, adj[1].0);
    init_make_portal(&mut commands, &pos_hash, adj[3].0, adj[1].0);
    add_edge_hash(&mut adj_hash, adj[3].0, adj[2].0);
    init_make_portal(&mut commands, &pos_hash, adj[3].0, adj[2].0);
    add_edge_hash(&mut adj_hash, adj[2].0, adj[4].0);
    init_make_portal(&mut commands, &pos_hash, adj[2].0, adj[4].0);


    println!("{:?}", adj);
    println!("{:?}", adj_hash);

    //get_route(&mut test_hash, adj[0].0, adj[3].0);

    let mut all_routes: HashMap<(Entity, Entity), VecDeque<Entity>> = HashMap::new();
    for key in adj_hash.keys() {
        for other in adj_hash.keys() {
            if key != other {
                all_routes.insert(
                    (*key, *other),
                    get_route(&adj_hash, *key, *other));
            }
        }
    }

    println!("{:?}", all_routes);


    commands.insert_resource(DebugLineMap {
        val: adj
    });
    map.routes = all_routes;
}

pub fn test_map_setup_fill(
    mut commands: Commands,
    cluster: Res<SystemMap>,
    map: Res<GalaxyMap>,
) {
    let list = &cluster.0;

    let center = commands.spawn((
        spawn_station_at(SimPosition(DVec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }), list[0]),
        UndockLoc,
    )).id();
    commands.spawn(spawn_inventory(center));

    let anom = commands.spawn((
        spawn_anom(SimPosition(DVec3 {
            x: au_to_system(-10.0),
            y: au_to_system(10.0),
            z: 0.0,
        }), list[1]),
        AnomalyMining { tracked: Vec::new() },
        RegisterTo(list[1]),
    )).id();

    let mine_in_anom = Steps::build()
        .label("MineInAnom")
        .step(MoveToAnom)
        .step(MineAnom)
        .step(DepositOre);


    let ship = commands.spawn((
        spawn_new_pilot(),
        UndockingFrom(center),
        WeaponBundle {
            _weapon: Weapon {
                _type: WeaponType::Mining,
                config: WeaponConfig::RangeShort,
                size: WeaponSize::Small,
                tier: 1,
                bank: 1,
            },
            target: WeaponTarget(None),
            in_range: WeaponInRange(false),
        },
        TravelTo(list[4])
    )).id();

    //get_system_in_range(2,&GalaxyCoordinate(list[0]),&map);
}

#[derive(Component)]
pub struct TakeGate {
    from: Entity,
    to: Entity,
}

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
            Destination(DestoType::DPosition(*gate_pos))
        )).remove::<ContinueRoute>();
    }
}


pub fn take_gate_system(
    mut commands: Commands,
    gate: Query<(&GalaxyCoordinate, &SimPosition)>,
    query: Query<(Entity, &GalaxyCoordinate, &SimPosition, &TravelRoute, &TakeGate)>,
) {
    for (id, coord, pos, route, order) in &query {
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
}

pub fn travel_route_system(
    mut commands: Commands,
    system: Query<(&SolarSystem)>,
    mut query: Query<(Entity, &GalaxyCoordinate, &SimPosition, &Destination, &mut TravelRoute), (Without<Warping>, Without<TakeGate>)>,
) {
    for (id, coord, pos, dest, mut route) in &mut query {
        match dest.0 {
            DestoType::None => {}
            DestoType::TEntity(_) => {}
            DestoType::DPosition(target) => {
                let dist: f64 = (pos.0 - target.0).length();
                if dist <= m_to_system(100.0) {
                    let travel_dir = route.route.pop_front().unwrap();
                    println!("current dir {:?}", travel_dir);
                    let (gate_from_id, gate_to_id) = system.get(coord.0).expect("system?").gates.get(&travel_dir).unwrap();
                    commands.entity(id).insert(
                        (
                            TakeGate {
                                from: *gate_from_id,
                                to: *gate_to_id,
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
                            DestoType::DPosition(*gate.get(*gate_from).unwrap().0)
                        )
                    )).remove::<TravelTo>();
            }
        }
    }
}


#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct RegisterGateToSystem {
    coord: Entity,
    gate_to: Entity,
}

fn init_make_portal(
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
                    visibility: Visibility { is_visible: false },
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
                    visibility: Visibility { is_visible: false },
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

fn add_edge(
    mut adj: &mut Vec<(Entity, Vec<usize>)>,
    a: usize,
    b: usize,
) {
    adj[a].1.push(b);
    adj[b].1.push(a);
}

fn add_edge_hash(
    mut _map: &mut HashMap<Entity, Vec<Entity>>,
    a: Entity,
    b: Entity,
) {
    _map.get_mut(&a).unwrap().push(b);
    _map.get_mut(&b).unwrap().push(a);
}

fn get_route(
    _map: &HashMap<Entity, Vec<Entity>>,
    from: Entity,
    to: Entity,
) -> VecDeque<Entity> {
    let mut visited: HashMap<Entity, bool> = HashMap::new();
    let mut queue: VecDeque<Entity> = VecDeque::new();
    let mut pred: HashMap<Entity, Option<Entity>> = HashMap::new();
    let mut route: VecDeque<Entity> = VecDeque::new();

    for key in _map.keys() {
        visited.insert(*key, false);
        pred.insert(*key, None);
    }
    visited.insert(from, true);
    queue.push_back(from);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        for neigh in _map.get(&current).unwrap().iter() {
            if !*visited.get(neigh).unwrap() {
                visited.insert(*neigh, true);
                pred.insert(*neigh, Some(current));
                queue.push_back(*neigh);

                if *neigh == to {
                    //println!("found");
                    break;
                }
            }
        }
    }

    let mut crawl = to;
    route.push_front(crawl);
    while *pred.get(&crawl).unwrap() != None {
        route.push_front(pred.get(&crawl).unwrap().unwrap());
        crawl = pred.get(&crawl).unwrap().unwrap();
    }
    return route;
}


#[derive(Resource)]
pub struct DebugLineMap {
    val: Vec<(Entity, Vec<usize>)>,
}

pub fn line_debug(
    state: Res<State<ViewState>>,
    _map: Res<DebugLineMap>,
    mut lines_draw: ResMut<DebugLines>,
    query: Query<&Transform>,
) {
    if *state.current() == GALAXY {
        for lines in _map.val.iter() {
            let _self = query.get(lines.0).unwrap();
            for line in lines.1.iter() {
                let to = query.get(_map.val[*line].0).unwrap();
                lines_draw.line(_self.translation, to.translation, 0.0);
            }
        }
    }
}