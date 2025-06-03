use bevy::prelude::*;

use super::GameState;

#[derive(Debug, Clone, Copy)]
pub(crate) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Settings), setup)
            .add_systems(OnExit(GameState::Settings), cleanup);
    }
}

fn setup() {
    info!("enter GameState::Settings");
    // TODO
}

fn cleanup() {
    info!("exit GameState::Settings");
    // TODO
}
