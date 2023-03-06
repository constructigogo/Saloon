use bevy::app::{App, PluginGroupBuilder};
use bevy::prelude::*;
use bevy::prelude::system_adapter::new;
use bevy::utils::tracing::callsite::register;

use crate::space::anomalies::*;
use crate::space::asteroid::asteroid_life_cycle_system;
use crate::space::inventory::{debug_items, register_inventory_to_ship_system, setup_world_inventory, update_cached_volume_system};
use crate::space::project::project_to_camera;
use crate::space::weapon::mining::resource_gathering_system;
use crate::transfer_item;

use self::galaxy::*;
use self::ship::*;
use self::weapon::*;

pub mod ship;
pub mod pilot;
pub mod galaxy;
pub mod project;
pub mod station;
pub mod anomalies;
pub mod asteroid;
pub mod celestial;
pub mod inventory;
pub mod weapon;

pub struct SpaceGamePlugins;

impl PluginGroup for SpaceGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GalaxyPlugin)
            .add(AnomPlugins)
            .add(ShipPlugins)
            .add(InventoryPlugins)
            .add(WeaponPlugins)
            .add(AsteroidPlugins)
    }
}

pub struct GalaxyPlugin;

impl Plugin for GalaxyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(ViewState::GALAXY)
            .insert_resource(SystemMap(Vec::new()))
            .insert_resource(GalaxyScale(0.000001))
            .insert_resource(CurrentSystemDisplay(None))
            .add_event::<HideGalaxyEvent>()
            .add_event::<HideSystemEvent>()
            .add_event::<RenderGalaxyEvent>()
            .add_event::<RenderSystemEvent>()
            .add_system(project_to_camera)
            .add_system(exit_system_view)
            .add_system(click_enter_system_view)
            .add_system(hide_galaxy_view)
            .add_system(hide_system_view)
            .add_system(add_to_system_view)
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
        app.add_system(register_anom);
        app.add_system(init_anom);
        app.add_system(mining_anomaly_lifecycle_system);
    }
}

pub struct InventoryPlugins;

impl Plugin for InventoryPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_world_inventory)
            .add_system(register_inventory_to_ship_system)
            .add_system(transfer_item)
            .add_system(update_cached_volume_system)
            .add_system(debug_items);
    }
}


pub struct WeaponPlugins;

impl Plugin for WeaponPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_system(weapon_init_system)
            .add_system(weapon_range_checker_system)
            .add_system(weapon_cooldown_ticking_system)
            .add_system(resource_gathering_system);
    }
}



pub struct AsteroidPlugins;

impl Plugin for AsteroidPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_system(asteroid_life_cycle_system);
    }
}

