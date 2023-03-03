use bevy::app::{App, PluginGroupBuilder};
use bevy::prelude::*;
use bevy::prelude::system_adapter::new;

use crate::space::inventory::debug_items;
use crate::space::project::project_to_camera;
use crate::transfer_item;

use self::galaxy::*;
use self::ship::*;

pub mod ship;
pub mod pilot;
pub mod galaxy;
pub mod project;
pub mod station;
pub mod anomalies;
pub mod asteroid;
pub mod celestial;
pub mod inventory;

pub struct SpaceGamePlugins;

impl PluginGroup for SpaceGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GalaxyPlugin)
            .add(ShipPlugins)
            .add(InventoryPlugins)
    }
}

pub struct GalaxyPlugin;

impl Plugin for GalaxyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(ViewState::GALAXY)
            .insert_resource(SystemMap(Vec::new()))
            .insert_resource(GalaxyScale(0.000001))
            .add_event::<HideGalaxyEvent>()
            .add_event::<HideSystemEvent>()
            .add_event::<RenderGalaxyEvent>()
            .add_event::<RenderSystemEvent>()
            .add_system(project_to_camera)
            .add_system(exit_system_view)
            .add_system(click_enter_system_view)
            .add_system(hide_galaxy_view)
            .add_system(hide_system_view)
            .add_system(flag_render_solar_system)
            .add_system(generate_galaxy_view)
            .add_system(generate_system_view);
    }
}

pub struct ShipPlugins;

impl Plugin for ShipPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_system(compute_ship_forces)
            .add_system(undock_pilot_system);
    }
}

pub struct AnomPlugins;

impl Plugin for AnomPlugins {
    fn build(&self, app: &mut App) {
        //app.add_system();
    }
}

pub struct InventoryPlugins;

impl Plugin for InventoryPlugins {
    fn build(&self, app: &mut App) {
        app.add_system(transfer_item)
            .add_system(debug_items);
    }
}


