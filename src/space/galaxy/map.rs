use std::process::id;

use bevy::prelude::*;
use bevy::utils::hashbrown::*;
use bevy_mod_picking::{PickableBundle, Selection};
use bevy_prototype_debug_lines::DebugLines;

use crate::{DVec3, m_to_system, MaterialMesh2dBundle, Rendered, SimPosition, SolarSystem, SystemMap, UndockLoc, ViewState};
use crate::ViewState::GALAXY;

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

        cluster.0.push(id);
    }

    add_edge(&mut adj, 0, 1);
    add_edge(&mut adj, 1, 2);
    add_edge(&mut adj, 3, 1);
    add_edge(&mut adj, 3, 2);
    //add_edge(&mut adj,2,0);

    println!("{:?}", adj);
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

#[derive(Resource)]
pub struct DebugLineMap {
    val: Vec<(Entity, Vec<usize>)>,
}

pub fn line_debug(
    state: Res<State<ViewState>>,
    _map : Res<DebugLineMap>,
    mut linesDraw: ResMut<DebugLines>,
    query : Query<&Transform>
) {
    if *state.current() == GALAXY  {
        for lines in _map.val.iter(){
            let _self = query.get(lines.0).unwrap();
            for line in lines.1.iter() {
                let to = query.get(_map.val[*line].0).unwrap();
                linesDraw.line(_self.translation, to.translation,0.0);
            }
        }
    }
}