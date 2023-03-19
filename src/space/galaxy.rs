pub mod map;
pub mod gates;
pub mod route;

use std::f64::consts::PI;
use std::ops::Range;
use std::thread::current;

use bevy::{ecs::{entity::Entities, query}, prelude::*};
use bevy::ecs::system::lifetimeless::SCommands;
use bevy::math::DVec3;
use bevy::utils::HashMap;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

pub fn m_to_system(from: f64) -> f64 {
    return from * 0.000001;
}

pub fn au_to_system(from : f64) -> f64 {
    return from * 1500.0;
}

pub fn around_pos(pos: SimPosition, radius: f64) -> SimPosition {
    let rng = thread_rng().gen::<f64>() * 2.0 * PI;
    let rad = thread_rng().gen_range::<f64, Range<f64>>(0.0..radius);
    let n_pos = pos.0 + ((DVec3::new(f64::cos(rng), f64::sin(rng), 0.0)) * m_to_system(rad));
    return SimPosition(n_pos);
}

/// Since we need every ship to be able to live in a different system/map
/// we need to simulate them independently of the rendering, all in local space
/// but without interfering with each others (ships in system A should not see ships in system B)
/// TODO WE NEED THAT ASAP
#[derive(Resource, Default)]
pub struct SystemMap(pub Vec<Entity>);

/// Index of the reference system
#[derive(Component, Deref)]
pub struct GalaxyCoordinate(pub Entity);

#[derive(Resource, Deref)]
pub struct GalaxyScale(pub f64);

/// Since coordinates are float we need to avoid going into large coordinates value
#[derive(Component, Default)]
pub struct SolarSystem {
    pub anomalies: Vec<Entity>,

    // k : destination system, v : (gate id, gate destination id)
    pub gates: HashMap<Entity,(Entity,Entity)>,
    //pub size: f32, //probably useless we'll see
}

/// Position for simulation
#[derive(Component, Default, Copy, Clone, Deref, DerefMut, Reflect)]
pub struct SimPosition(pub DVec3);

#[derive(Bundle)]
pub struct GalaxySpawnGateBundle {
    pub tag: GalaxyGateTag,
    pub disp : DisplayableGalaxyEntityBundle,
}

// Destination system
#[derive(Component, Deref)]
pub struct GateDestination(pub Entity);


#[derive(Component)]
pub struct GalaxyGateTag;


#[derive(Bundle)]
pub struct DisplayableGalaxyEntityBundle {
    pub display: SpriteBundle,
    pub galaxy: GalaxyEntityBundle,
}

#[derive(Bundle)]
pub struct GalaxyEntityBundle {
    pub galaxy_coord: GalaxyCoordinate,
    pub simulation_position: SimPosition,
}
