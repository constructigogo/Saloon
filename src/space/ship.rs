use bevy::math::Vec3Swizzles;
use bevy::{prelude::*, ecs::component, transform::components};
use rand::prelude::*;
use crate::base::velocity::*;
use crate::space::pilot::*;

#[path = "../base/velocity.rs"] 
pub mod velocity;

#[path = "../space/pilot.rs"] 
pub mod pilot;

 
pub struct ShipPlugins;
impl Plugin for ShipPlugins {
    fn build(&self, app: &mut App) {
        app
        .add_system(compute_ship_movement)
        .add_system(undock_pilot_system);
    }
}

///TODO should schedule only a few times per frame
pub fn compute_ship_movement(
    time : Res<Time>,
    mut query: Query<(&mut Velocity ,&Transform, &Destination, &Mass , &ThrusterEngine,)>){
    for (mut vel,transform, dest, mass, thruster) in &mut query {
        let d_type : &DestoType = &dest.0;
        match d_type {
            DestoType::DPosition(vec)  => {
                let dir = (*vec - transform.translation.truncate()).try_normalize();
                match dir {
                    Some(d) => {
                        vel.0 += d * (get_accel(mass, thruster) * time.delta_seconds());
                    },
                    None => {},
                }
            },
            DestoType::TEntity(ent) => {},
            _ => {}
        }

    }
}

fn get_accel(m : &Mass, th : &ThrusterEngine) -> f32{
    return (th.thrust/m.0) as f32;
}


pub fn undock_pilot_system(mut commands: Commands,
    query: Query<(Entity, With<FlagUndocking>)>) {
    let mut rng = rand::thread_rng();


    for (entity,_) in query.iter()  {
        commands.entity(entity).insert(
            ShipBundle {
                display: SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.25, 0.25, 0.75),
                        custom_size: Some(Vec2::new(16.0, 16.0)),
                        ..default()
                    },
                    transform : Transform{
                        translation: Vec3 { 
                            x: rng.gen_range(-200.0..200.0),
                            y: rng.gen_range(-150.0..150.0),
                            z: 0.0 },
                        ..default()
                    },
                    ..default()
                },
                movable: MovableBundle { 
                    mass: Mass(100000), 
                    velocity: Velocity::default(), 
                    thruster: ThrusterEngine { 
                        thrust: 500000,
                        angular: 25.15 
                    }, 
                    move_towards: Destination(DestoType::DPosition(Vec2::ZERO))
                },
            }
        ).remove::<FlagUndocking>();
    }
}



///Flag to schedule a ship undock during the next frame
#[derive(Component)]
pub struct FlagUndocking;

#[derive(Bundle)]
pub struct ShipBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    display : SpriteBundle,
    movable : MovableBundle,
}

///Anything movable should be made with this bundle
#[derive(Bundle)]
pub(crate) struct MovableBundle {
    pub mass : Mass,
    pub velocity : Velocity,
    pub thruster : ThrusterEngine,
    pub move_towards : Destination,
}



#[derive(Bundle)]
pub struct ShipStatsBundle {
    healthComp : Health
    
}

#[derive(Bundle)]
pub struct DamageableBundle {

    health : Health,
    
}


/// Mass in Kg of an entity
#[derive(Component,Deref, DerefMut)]
pub struct Mass(u64);



#[derive(Component)]
pub struct ThrusterEngine{
    ///Thrust in Newton (N)
    thrust : u64,
    ///Angular in degree/sec
    angular : f32,
}

#[derive(Component)]
pub struct WarpEngine{
    range : f64,
    speed : f64,
    power : f64,
}


#[derive(Component)]
pub struct Health{
    current_structure : f32,
    max_structure : f32,
    current_armor : f32,
    max_armor : f32,
    current_shield : f32,
    max_shield : f32,
}

#[derive(Default)]
pub enum DestoType {
    DPosition(Vec2),
    TEntity(Entity),
    #[default]
    None
}


#[derive(Component, Deref, DerefMut)]
pub struct Destination(DestoType);

