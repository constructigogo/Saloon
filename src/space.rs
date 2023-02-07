use bevy::app::{App, PluginGroupBuilder};
use bevy::prelude::*;

use self::ship::*;

pub mod ship;
pub mod pilot;

pub struct SpaceGamePlugins;
impl PluginGroup for SpaceGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ShipPlugins)
    }
}



