use bevy::{ecs::component, prelude::*, transform::components};
use bevy::math::{DVec2, DVec3, Vec3Swizzles};
use rand::prelude::*;

use crate::base::velocity::*;
use crate::space::galaxy::SimPosition;
use crate::space::pilot::*;

use super::galaxy::GalaxyCoordinate;

#[path = "../base/velocity.rs"]
pub mod velocity;

#[path = "../space/pilot.rs"]
pub mod pilot;


pub struct ShipPlugins;

impl Plugin for ShipPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_system(compute_ship_forces)
            .add_system(undock_pilot_system);
    }
}

///TODO should schedule only a few times per frame
pub fn compute_ship_forces(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &SimPosition, &Destination, &Mass, &ThrusterEngine, )>) {
    for (mut vel, sPos, dest, mass, thruster) in &mut query {
        let d_type: &DestoType = &dest.0;
        let u: Option<DVec2> = DVec2 { x: vel.x, y: vel.y }.try_normalize();


        match d_type {
            DestoType::DPosition(vec) => {
                let res = get_delta_velocity(
                    vec,
                    &sPos.truncate(),
                    mass,
                    thruster,
                    time.delta_seconds_f64(),
                );
                match u {
                    None => {}
                    Some(uv) => {
                        let drag_coef = 0.5 * vel.0.length_squared() * 0.225;
                        let drag_vec: DVec2 = -uv * drag_coef;
                        vel.0 += drag_vec * time.delta_seconds_f64();
                    }
                }
                match res {
                    None => {}
                    Some(acc_dt) => { vel.0 += acc_dt }
                }
            }
            DestoType::TEntity(ent) => {
                let res = get_delta_velocity(
                    &ent.0.truncate(),
                    &sPos.truncate(),
                    mass,
                    thruster,
                    time.delta_seconds_f64(),
                );

                match u {
                    None => {}
                    Some(uv) => {
                        let drag_coef = 0.5 * vel.0.length_squared() * 0.225;
                        let drag_vec: DVec2 = -uv * drag_coef;
                        vel.0 += drag_vec * time.delta_seconds_f64();
                    }
                }
                match res {
                    None => {}
                    Some(acc_dt) => { vel.0 += acc_dt }
                }
            }
            _ => {}
        }
    }
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

fn get_accel(m: &Mass, th: &ThrusterEngine) -> f64 {
    return (th.thrust / m.0) as f64;
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct UndockLoc;


pub fn undock_pilot_system(
    mut commands: Commands,
    query: Query<(Entity, &UndockingFrom)>,
    undocks: Query<&SimPosition, With<UndockLoc>>) {
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
                        coordinate: GalaxyCoordinate(from.0),
                        simulation_position: SimPosition(trans.0*3.0),
                        mass: Mass(100000),
                        velocity: Velocity::default(),
                        thruster: ThrusterEngine {
                            max_speed: 10.0,
                            thrust: 1500000,
                            angular: 25.15,
                        },
                        move_towards: Destination(DestoType::DPosition(DVec2 {
                            x: rng.gen_range(-200.0..200.0),
                            y: rng.gen_range(-150.0..150.0),
                        })),
                    },
                }
            ).remove::<UndockingFrom>();
        } else {

        }
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
    max_speed: f32,
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
    DPosition(DVec2),
    TEntity(SimPosition),
    #[default]
    None,
}


#[derive(Component, Deref, DerefMut)]
pub struct Destination(pub DestoType);

