use bevy::{prelude::*, ecs::component, transform::components};
use crate::base::velocity::*;


pub fn spawn_new_pilot() -> PilotBundle{
    return PilotBundle{
        _pilot : Pilot{
            level : 1,
            u_id : 0,
        },
        respawn_base : RespawnBase(None),
        pilot_name: EName("ZEZRRTERT".to_string()),
        pilot_faction: Faction(0),
    }
}


#[derive(Bundle)]
pub struct PilotBundle {
    pub _pilot : Pilot,
    pub respawn_base : RespawnBase,
    pub pilot_name : EName,
    pub pilot_faction : Faction
}

#[derive(Component,Deref, DerefMut)]
pub struct RespawnBase(pub Option<Entity>);

#[derive(Component,Deref, DerefMut)]
pub struct EName(pub String);

#[derive(Component,Deref, DerefMut)]
pub struct Faction(pub u32);

#[derive(Component)]
pub struct Pilot{
    pub level : u8,
    pub u_id : u64,
}

#[derive(Component,Deref, DerefMut)]
pub struct PilotLevel(u8);

#[derive(Component,Deref, DerefMut)]
pub struct PilotUID(u64);
