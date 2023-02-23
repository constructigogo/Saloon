use bevy::app::{App, PluginGroupBuilder};
use bevy::prelude::*;

use self::camera::CameraControllerPlugin;
use self::settings::*;
use self::velocity::VelocityPlugin;

pub mod timer;
pub mod velocity;
pub mod camera;
pub mod settings;


pub fn frame_update(time: Res<Time>) {
    info!(
        "time since last frame_update: {:?}",
        time.delta()
    );
}


pub struct BaseLogicPlugins;

impl PluginGroup for BaseLogicPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GameSettingsPlugin)
            .add(CameraControllerPlugin)
            .add(VelocityPlugin)
    }
}

