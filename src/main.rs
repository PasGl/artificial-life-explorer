use bevy::prelude::*;
use bevy_egui::EguiPlugin;
mod cellular_automata;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Artificial Life Explorer".into(),
                    present_mode: bevy::window::PresentMode::AutoNoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    window_theme: Some(bevy::window::WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
            EguiPlugin,
            //bevy::diagnostic::LogDiagnosticsPlugin::default(),
            //bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        ))
        .insert_resource(cellular_automata::state::CellularSystemState::default())
        .insert_resource(cellular_automata::state::HeightMapMesh::default())
        .add_systems(Startup, cellular_automata::setup_3d_scene)
        .add_systems(
            Update,
            (
                cellular_automata::state::next_iteration,
                cellular_automata::egui_system,
                cellular_automata::height_map::update_heightmap,
            ),
        )
        .run();
}
