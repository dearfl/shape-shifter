use bevy::prelude::*;

use crate::game::GamePhase;

#[derive(Debug, Clone, Copy)]
pub(crate) struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePhase::Upgrade), setup)
            .add_systems(OnExit(GamePhase::Upgrade), cleanup);
    }
}

fn setup(mut state: ResMut<NextState<GamePhase>>) {
    info!("enter GamePhase::Upgrade");
    // TODO
    state.set(GamePhase::Battle);
}

fn cleanup() {
    info!("exit GamePhase::Upgrade");
    // TODO
}
