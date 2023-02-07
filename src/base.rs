use bevy::app::{App, PluginGroupBuilder};
use bevy::prelude::*;

use self::velocity::VelocityPlugin;


pub mod timer;
pub mod velocity;




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
            .add(VelocityPlugin)
    }
}
