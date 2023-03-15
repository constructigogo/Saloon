use bevy::{ecs::component, prelude::*, transform::components};

use crate::base::velocity::*;
use crate::space::empire::Faction;

pub fn spawn_new_pilot() -> PilotBundle {
    return PilotBundle {
        _pilot: PilotStats {
            level: 1,
            u_id: 0,
        },
        respawn_base: RespawnBase(None),
        pilot_name: EName("ZEZRRTERT".to_string()),
        pilot_faction: Faction(0),
    };
}


#[derive(Bundle)]
pub struct PilotBundle {
    pub _pilot: PilotStats,
    pub respawn_base: RespawnBase,
    pub pilot_name: EName,
    pub pilot_faction: Faction,
}

#[derive(Component, Deref, DerefMut)]
pub struct RespawnBase(pub Option<Entity>);

#[derive(Component, Deref, DerefMut)]
pub struct EName(pub String);



#[derive(Component)]
pub struct PilotStats {
    pub level: u8,
    pub u_id: u64,
}

#[derive(Component, Deref, DerefMut)]
pub struct PilotLevel(u8);

#[derive(Component, Deref, DerefMut)]
pub struct PilotUID(u64);
