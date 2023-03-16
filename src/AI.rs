use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use big_brain::BigBrainStage;

use crate::AI::miner::*;

pub mod miner;
pub mod utils;
pub mod general;

pub struct AIPlugins;

impl PluginGroup for AIPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(MinerAIPlugin)
    }
}

pub struct MinerAIPlugin;

impl Plugin for MinerAIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_to_stage(BigBrainStage::Actions, move_to_anom_system)
            .add_system_to_stage(BigBrainStage::Actions, mine_anom_system)
            .add_system_to_stage(BigBrainStage::Actions, deposit_ore_action_system)
            .add_system_to_stage(BigBrainStage::Scorers, mine_scorer_system)
        ;
    }
}