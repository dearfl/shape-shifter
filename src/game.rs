use bevy::{prelude::*, render::camera::ScalingMode};

use exit::ExitPlugin;
use ingame::InGamePlugin;
use menu::MenuPlugin;
use settings::SettingsPlugin;

mod exit;
mod ingame;
mod menu;
mod settings;

// a marker component used for cleanup
#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct Selected;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub(crate) enum GameState {
    #[default]
    Menu,
    Settings,
    Ingame,
    Exit,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, SubStates)]
#[source(GameState = GameState::Ingame)]
pub(crate) enum GamePhase {
    #[default]
    Battle,
    Upgrade,
    // Other events
}

pub(crate) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // fullscreen request is ignored on hyprland
                #[cfg(not(target_family = "wasm"))]
                mode: bevy::window::WindowMode::Fullscreen(
                    MonitorSelection::Index(0), // Current/Primary crashes on wayland in 0.16.0
                    VideoModeSelection::Current,
                ),
                title: "Shape Shifter".to_string(),
                name: Some("Bevy shape shifter".to_string()),
                resizable: false,
                decorations: false,
                transparent: false,
                focused: true,
                ..Default::default()
            }),
            ..Default::default()
        }));

        #[cfg(feature = "inspect")]
        app.add_plugins((
            bevy_screen_diagnostics::ScreenFrameDiagnosticsPlugin,
            bevy_screen_diagnostics::ScreenDiagnosticsPlugin::default(),
            bevy_inspector_egui::bevy_egui::EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
        ));

        app.init_state::<GameState>()
            .add_sub_state::<GamePhase>()
            .add_plugins((MenuPlugin, SettingsPlugin, InGamePlugin, ExitPlugin))
            .add_systems(Startup, setup);
    }
}

fn setup(mut cmd: Commands) {
    info!("spawn camera2d");
    cmd.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 100.0,
            },
            ..OrthographicProjection::default_2d()
        }),
        Camera {
            hdr: true,
            ..Default::default()
        },
        IsDefaultUiCamera,
        Msaa::Off,
    ));
}
