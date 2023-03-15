use std::collections::VecDeque;
use std::process::id;

use bevy::prelude::*;
use bevy::utils::hashbrown::*;
use bevy_mod_picking::{PickableBundle, Selection};
use bevy_prototype_debug_lines::DebugLines;

use crate::{DVec3, m_to_system, MaterialMesh2dBundle, Rendered, SimPosition, SolarSystem, SystemMap, UndockLoc, ViewState};
use crate::ViewState::GALAXY;

#[derive(Component, Deref)]
pub struct TravelRoute{
    route : VecDeque<Entity>,
}

#[derive(Component, Deref)]
pub struct TravelTo(pub Entity);


struct GalaxyMap {
    //Every system
    node: Vec<Entity>,
    //lane between systems
    lane: Vec<(Entity, Entity)>,
    routes: HashMap<HashRoute, Route>,
}

#[derive(Eq, PartialEq, Hash)]
struct HashRoute {
    from: Entity,
    to: Entity,
}

struct Route {
    steps: Vec<Entity>,
}

pub fn generate_map_pathfinding_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cluster: ResMut<SystemMap>,
) {
    let positions: [DVec3; 4] = [
        DVec3::new(-150.0, -75.0, 0.0) * m_to_system(10.0),
        DVec3::new(0.0, 75.0, 0.0) * m_to_system(10.0),
        DVec3::new(150.0, -75.0, 0.0) * m_to_system(10.0),
        DVec3::new(150.0, 75.0, 0.0) * m_to_system(10.0),
    ];

    let mut adj: Vec<(Entity, Vec<usize>)> = Vec::new();

    let mut test_hash: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for pos in positions.iter() {
        let id = commands.spawn(
            (
                SolarSystem {
                    anomalies: Vec::new(),
                    gates: Vec::new(),
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
        test_hash.insert(id, Vec::new());

        cluster.0.push(id);
    }

    add_edge(&mut adj, 0, 1);
    add_edge(&mut adj, 1, 2);
    add_edge(&mut adj, 3, 1);
    add_edge(&mut adj, 3, 2);
    //add_edge(&mut adj,2,0);
    add_edge_hash(&mut test_hash, adj[0].0, adj[1].0);
    add_edge_hash(&mut test_hash, adj[1].0, adj[2].0);
    add_edge_hash(&mut test_hash, adj[3].0, adj[1].0);
    add_edge_hash(&mut test_hash, adj[3].0, adj[2].0);


    println!("{:?}", adj);
    println!("{:?}", test_hash);

    let mut RouteList: HashMap<(Entity, Entity), Route> = HashMap::new();

    get_route(&mut test_hash, adj[0].0, adj[3].0);

    let mut all_routes : HashMap<(Entity, Entity), VecDeque<Entity>> = HashMap::new();

    for key in test_hash.keys() {
        for other in test_hash.keys() {
            if key != other {
                all_routes.insert(
                    (*key,*other),
                    get_route(&test_hash,*key,*other));
            }
        }
    }

    println!("{:?}", all_routes);

    commands.insert_resource(DebugLineMap {
        val: adj
    });
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
    _map: & HashMap<Entity, Vec<Entity>>,
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

    //println!("{:?}", route);
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