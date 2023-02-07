use bevy::{prelude::*, ecs::component, transform::components};
use crate::base::velocity::*;

#[path = "../base/velocity.rs"] 
pub mod velocity;
 
pub fn UndockPilotSystem(mut commands: Commands,
    query: Query<(Entity, With<FlagUndocking>)>) {
    
    for (entity,_) in query.iter()  {
        commands.entity(entity).insert(
            ShipBundle {
                display: SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.25, 0.25, 0.75),
                        custom_size: Some(Vec2::new(16.0, 16.0)),
                        ..default()
                    },
                    ..default()
                },
                movable: MovableBundle { 
                    mass: Mass(1000.0), 
                    velocity: Velocity::default(), 
                    thruster: ThrusterEngine { 
                        thrust: 200000, 
                        angular: 25.15 
                    }, 
                    move_towards: Destination(Vec2::ZERO)
                },
            }
        ).remove::<FlagUndocking>();
    }
}

pub fn SpawnNewPilot() -> PilotBundle{
    return PilotBundle{
        _pilot : Pilot{
            level : 1,
            u_id : 0,
        },
        pilot_name: Name("ZEZRRTERT".to_string()),
        pilot_faction: Faction(0),
    }
}

#[derive(Component)]
pub struct FlagUndocking;

#[derive(Bundle)]
pub struct ShipBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    display : SpriteBundle,
    movable : MovableBundle,
}


#[derive(Bundle)]
pub(crate) struct MovableBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    pub mass : Mass,
    pub velocity : Velocity,
    pub thruster : ThrusterEngine,
    pub move_towards : Destination,
}



#[derive(Bundle)]
pub struct ShipStatsBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality

    healthComp : Health
    
}

#[derive(Bundle)]
pub struct DamageableBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    health : Health,
    
}

#[derive(Component,Deref, DerefMut)]
pub struct Mass(f32);


#[derive(Component)]
pub struct ThrusterEngine{
    thrust : u32,
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
    currentStructure : f32,
    maxStructure : f32,
    currentArmor : f32,
    maxArmor : f32,
    currentShield : f32,
    maxShield : f32,
}




#[derive(Component,Deref, DerefMut)]
pub struct Destination(Vec2);


#[derive(Bundle)]
pub struct PilotBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    _pilot : Pilot,
    pilot_name : Name,
    pilot_faction : Faction
}

#[derive(Component,Deref, DerefMut)]
pub struct Name(String);

#[derive(Component,Deref, DerefMut)]
pub struct Faction(u32);

#[derive(Component)]
pub struct Pilot{
    level : u8,
    u_id : u64,
}

#[derive(Component,Deref, DerefMut)]
pub struct PilotLevel(u8);

#[derive(Component,Deref, DerefMut)]
pub struct PilotUID(u64);
