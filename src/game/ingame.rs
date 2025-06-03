use bevy::prelude::*;

use super::GameState;
use battle::BattlePlugin;
use upgrade::UpgradePlugin;

mod battle;
mod upgrade;

#[derive(Debug, Clone, Copy)]
pub(crate) struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BattlePlugin, UpgradePlugin))
            .add_systems(OnEnter(GameState::Ingame), setup)
            .add_systems(OnExit(GameState::Ingame), cleanup);
    }
}

fn setup() {
    info!("enter GameState::InGame");
    // TODO
}

fn cleanup() {
    info!("exit GameState::InGame");
    // TODO
}
