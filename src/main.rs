use bevy::prelude::*;
use bevy_egui::EguiPlugin;
mod cellular_automata;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin,
            //bevy::diagnostic::LogDiagnosticsPlugin::default(),
            //bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        ))
        .insert_resource(cellular_automata::state::CellularSystemState::default())
        .insert_resource(cellular_automata::state::HeightMapMesh::default())
        .insert_resource(FixedTime::new_from_secs(1.0 / 50.0))
        .add_systems(Startup, cellular_automata::PixelMorphApp::setup_3d_scene)
        .add_systems(
            FixedUpdate,
            (
                cellular_automata::state::next_iteration,
                cellular_automata::PixelMorphApp::update_3d_scene,
            ),
        )
        .add_systems(Update, cellular_automata::PixelMorphApp::egui_system)
        .run();
}
