use bevy::app::{App, PluginGroupBuilder};
use bevy::prelude::*;
use bevy::prelude::system_adapter::new;

use self::galaxy::*;
use self::ship::*;

pub mod ship;
pub mod pilot;
pub mod galaxy;

pub struct SpaceGamePlugins;
impl PluginGroup for SpaceGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
        .add(GalaxyPlugin)
    }
}

pub struct GalaxyPlugin;
impl Plugin for GalaxyPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(SystemMap(Vec::new()))
        .add_system(flag_render_solar_system)
        .add_system(generate_view);
    }
}

