use bevy::prelude::*;



///Velocity of an entity, in m/s
#[derive(Component,Default, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

fn apply_velocity(time: Res<Time>,mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}



pub struct VelocityPlugin;
impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_velocity);
    }
}