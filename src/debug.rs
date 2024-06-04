use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use iyes_perf_ui::PerfUiCompleteBundle;
use iyes_perf_ui::PerfUiPlugin;

use crate::state::GameState;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.add_systems(OnEnter(GameState::InGame), debug_show_perf_stats);

        app.add_plugins(WorldInspectorPlugin::new())
            .add_plugins(PerfUiPlugin)
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin);
    }
}

#[cfg(debug_assertions)]
fn debug_show_perf_stats(mut commands: Commands) {
    commands.spawn(PerfUiCompleteBundle::default());
}
