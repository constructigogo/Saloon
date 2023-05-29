use bevy::app::{App, PluginGroupBuilder};
use bevy::prelude::*;
use bevy::prelude::system_adapter::new;
use bevy::time::common_conditions::on_fixed_timer;
use bevy::utils::tracing::callsite::register;

use crate::{generate_map_pathfinding_system, HashMap, transfer_item};
use crate::camera::{camera_input, CameraControllerPlugin};
use crate::gates::{register_gate_system, take_gate_added_system, take_gate_system};
use crate::map::{GalaxyMap, test_map_setup_fill};
use crate::route::{continue_route_system, on_travel_added, travel_route_system};
use crate::space::anomalies::*;
use crate::space::asteroid::asteroid_life_cycle_system;
use crate::space::inventory::{debug_items, register_inventory_to_ship_system, setup_world_inventory, update_cached_volume_system};
use crate::space::project::project_to_camera;
use crate::space::weapon::mining::resource_gathering_system;
use crate::warp::{check_for_warp_system, init_warp_system, warp_movement_system};

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
pub mod utils;
pub mod empire;

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
            .insert_resource(SystemMap(Vec::new()))
            .insert_resource(GalaxyScale(0.000001))
            .insert_resource(GalaxyMap{ routes: Default::default() })
            .add_system(register_gate_system)
            .add_system(
                project_to_camera
                    .after(camera_input)
            )
            .add_system(on_travel_added)
            .add_system(travel_route_system)
            .add_system(continue_route_system)
            .add_system(take_gate_added_system)
            .add_system(take_gate_system.in_schedule(CoreSchedule::FixedUpdate))

            //.add_system(take_gate_system.with_run_criteria(FixedTimestep::step(1.0)))
            .add_startup_system(generate_map_pathfinding_system)
            .add_startup_system(test_map_setup_fill.after(generate_map_pathfinding_system));
    }
}

pub struct ShipPlugins;

impl Plugin for ShipPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_system(compute_ship_forces)
            .add_system(init_warp_system)
            .add_system(check_for_warp_system)
            .add_system(warp_movement_system)
            .add_system(undock_pilot_system);
    }
}

pub struct AnomPlugins;

impl Plugin for AnomPlugins {
    fn build(&self, app: &mut App) {
        app.add_system(register_anom);
        app.add_system(anomaly_respawn_timer_system);
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

