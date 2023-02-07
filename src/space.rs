use bevy::app::{App, PluginGroupBuilder};
use bevy::prelude::*;

use self::ship::UndockPilotSystem;

pub mod ship;

pub struct SpaceGamePlugins;
impl PluginGroup for SpaceGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ShipPlugins)
    }
}


pub struct ShipPlugins;
impl Plugin for ShipPlugins {
    fn build(&self, app: &mut App) {
        app.add_system(UndockPilotSystem);
    }
}
