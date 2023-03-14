use bevy::math::DVec2;
use bevy::prelude::*;

use crate::space::galaxy::{GalaxyScale, SimPosition};
use crate::warp::Warping;

///Velocity of an entity, in m/s
#[derive(Component, Default, Deref, DerefMut)]
pub struct Velocity(pub DVec2);

//TODO implement some real space drag / max speed for ships
fn apply_velocity(time: Res<Time>,
                  scale: Res<GalaxyScale>,
                  mut query: Query<(&mut SimPosition, &Velocity),Without<Warping>>) {
    for (mut sPos, velocity) in &mut query {
        sPos.0.x += velocity.x * time.delta_seconds_f64() * scale.0;
        sPos.0.y += velocity.y * time.delta_seconds_f64() * scale.0;
    }
}


pub struct VelocityPlugin;
impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_velocity);
    }
}