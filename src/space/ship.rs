use std::cmp::max;

use bevy::{ecs::component, prelude::*, transform::components};
use bevy::math::{DVec2, DVec3, Vec3Swizzles};
use rand::prelude::*;

use crate::base::velocity::*;
use crate::space::galaxy::SimPosition;
use crate::space::pilot::*;
use crate::warp::Warping;

use super::galaxy::GalaxyCoordinate;

#[path = "../base/velocity.rs"]
pub mod velocity;

#[path = "../space/pilot.rs"]
pub mod pilot;
pub mod warp;


///TODO should schedule only a few times per frame
pub fn compute_ship_forces(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &SimPosition, &Destination, &Mass, &ThrusterEngine), Without<Warping>>) {
    query.par_for_each_mut(8, |(mut vel, sPos, dest, mass, thruster)|
        {
            let desto_type: &DestoType = &dest.0;
            let direction: Option<DVec2> = DVec2 { x: vel.x, y: vel.y }.try_normalize();
            let amplitude: f64 = vel.length();
            let accel: f64 = get_accel(mass, thruster);
            let drag: DVec2;

            match direction {
                None => { drag = DVec2::ZERO }
                Some(dir) => {
                    drag = -dir * (0.02 * 0.35 * ((amplitude * amplitude)));
                }
            }

            let mut thrust_dir: Option<DVec2> = None;
            let dist: f64;

            match desto_type {
                DestoType::DPosition(dPos) => {
                    //println!("desto : {:?}",dPos);

                    dist = (dPos.0.truncate() - sPos.0.truncate()).length() / 0.000001;
                    thrust_dir = Some((dPos.0.truncate() - sPos.0.truncate()).normalize());
                }
                DestoType::TEntity(dPos) => {
                    //println!("desto : {:?}",dPos.0.truncate());

                    dist = (dPos.0.truncate() - sPos.0.truncate()).length() / 0.000001;
                    thrust_dir = Some((dPos.0.truncate() - sPos.0.truncate()).normalize());
                }
                DestoType::None => {
                    dist = 0.0;
                }
            }


            match thrust_dir {
                None => {
                    let local_vel = vel.0;
                    vel.0 += (drag - local_vel.normalize() * accel) * time.delta_seconds_f64()
                }
                Some(dir) => {
                    let local_vel: DVec2 = vel.0;
                    let brake = dist / accel < local_vel.length() / accel;

                    if brake {
                        vel.0 += (drag - local_vel.normalize() * accel) * time.delta_seconds_f64()
                    } else {
                        vel.0 += ((drag +
                            dir * accel
                        )) * time.delta_seconds_f64();
                    }
                    //println!("vel  = {:?}, accel = {:?}, drag = {:?}, dist = {:?}, value = {:?}", amplitude, accel, drag.length(),dist, 0.0);
                }
            }
        });
}

fn get_delta_velocity(from: &DVec2, to: &DVec2, m: &Mass, th: &ThrusterEngine, dt: f64) -> Option<DVec2> {
    let dir = (*from - *to).try_normalize();
    match dir {
        Some(d) => {
            return Some(d * (get_accel(m, th) * dt));
        }
        None => {
            return None;
        }
    }
}

#[inline]
fn get_accel(m: &Mass, th: &ThrusterEngine) -> f64 {
    return (th.thrust / m.0) as f64;
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct UndockLoc;


pub fn undock_pilot_system(
    mut commands: Commands,
    query: Query<(Entity, &UndockingFrom)>,
    undocks: Query<(&SimPosition, &GalaxyCoordinate), With<UndockLoc>>) {
    let mut rng = thread_rng();
    for (entity, from) in query.iter() {
        if let Ok(trans) = undocks.get(commands.entity(from.0).id()) {
            commands.entity(entity).insert(
                ShipBundle {
                    display: SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.25, 0.25, 0.75),
                            custom_size: Some(Vec2::new(16.0, 16.0)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::ZERO,
                            ..default()
                        },
                        visibility: Visibility { is_visible: false },
                        ..default()
                    },
                    movable: MovableBundle {
                        coordinate: GalaxyCoordinate(trans.1.0),
                        simulation_position: SimPosition(trans.0.0),
                        mass: Mass(1500000),
                        velocity: Velocity::default(),
                        thruster: ThrusterEngine {
                            max_speed: 100.0,
                            thrust: 100000000,
                            angular: 25.15,
                        },
                        move_towards: Destination(DestoType::DPosition(SimPosition(DVec3 {
                            x: rng.gen_range(-0.0002..0.0002),
                            y: rng.gen_range(-0.00015..0.00015),
                            z: 0.0,
                        }))),
                    },
                }
            ).remove::<UndockingFrom>();
        } else { println!("invalid pos") }
    }
}


///Flag to schedule a ship undock during the next frame
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct UndockingFrom(pub Entity);

#[derive(Bundle)]
pub struct ShipBundle {
    display: SpriteBundle,
    movable: MovableBundle,
}

///Anything movable should be made with this bundle
#[derive(Bundle)]
pub(crate) struct MovableBundle {
    pub coordinate: GalaxyCoordinate,
    pub simulation_position: SimPosition,
    pub mass: Mass,
    pub velocity: Velocity,
    pub thruster: ThrusterEngine,
    pub move_towards: Destination,
}


#[derive(Bundle)]
pub struct ShipStatsBundle {
    health_comp: Health,
}

#[derive(Bundle)]
pub struct DamageableBundle {
    health: Health,
}


/// Mass in Kg of an entity
#[derive(Component, Deref, DerefMut)]
pub struct Mass(u64);


#[derive(Component)]
pub struct ThrusterEngine {
    // m/s
    max_speed: f64,
    ///Thrust in Newton (N)
    thrust: u64,
    ///Angular in degree/sec
    angular: f32,
}

#[derive(Component)]
pub struct WarpEngine {
    range: f64,
    speed: f64,
    power: f64,
}


#[derive(Component)]
pub struct Health {
    current_structure: f32,
    max_structure: f32,
    current_armor: f32,
    max_armor: f32,
    current_shield: f32,
    max_shield: f32,
}

#[derive(Default)]
pub enum DestoType {
    DPosition(SimPosition),
    TEntity(SimPosition),
    #[default]
    None,
}


#[derive(Component, Deref, DerefMut)]
pub struct Destination(pub DestoType);

