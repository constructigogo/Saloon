use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use big_brain::BigBrainSet;

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
            .add_system(move_to_anom_system.in_set(BigBrainSet::Actions))
            .add_system(mine_anom_system.in_set(BigBrainSet::Actions))
            .add_system(deposit_ore_action_system.in_set(BigBrainSet::Actions))
            .add_system(mine_scorer_system.in_set(BigBrainSet::Scorers))
        ;
    }
}