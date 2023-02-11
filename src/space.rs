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
        .add(ShipPlugins)
    }
}

pub struct GalaxyPlugin;
impl Plugin for GalaxyPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(SystemMap(Vec::new()))
        .insert_resource(VIEW_STATE::GALAXY)
        .add_event::<HideGalaxyEvent>()
        .add_event::<HideSystemEvent>()
        .add_event::<RenderGalaxyEvent>()
        .add_event::<RenderSystemEvent>()
        .add_system(exit_system_view)
        .add_system(click_enter_system_view)
        .add_system(flag_render_solar_system)
        .add_system(hide_galaxy_view)
        .add_system(hide_system_view)
        .add_system(generate_galaxy_view)
        .add_system(generate_system_view);
    }
}

