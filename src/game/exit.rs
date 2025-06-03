use bevy::prelude::*;

use super::GameState;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ExitPlugin;

impl Plugin for ExitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Exit), setup)
            .add_systems(OnExit(GameState::Exit), cleanup);
    }
}

fn setup(mut exit: EventWriter<AppExit>) {
    info!("enter GameState::Exit");
    exit.write(AppExit::Success);
    // TODO
}

fn cleanup() {
    info!("exit GameState::Exit");
    // TODO
}
