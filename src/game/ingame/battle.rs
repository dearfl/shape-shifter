use bevy::prelude::*;

use crate::game::GamePhase;

#[derive(Debug, Clone, Copy)]
pub(crate) struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePhase::Battle), setup)
            .add_systems(OnExit(GamePhase::Battle), cleanup);
    }
}

fn setup() {
    info!("enter GamePhase::Battle");
    // TODO
}

fn cleanup() {
    info!("exit GamePhase::Battle");
    // TODO
}
