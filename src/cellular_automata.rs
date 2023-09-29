use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
mod height_map;
pub(crate) mod state;
mod torus_topology;

pub struct PixelMorphApp {
    pub params: state::CellularSystemState,
}

impl PixelMorphApp {
    pub fn egui_system(mut contexts: EguiContexts, mut params: ResMut<state::CellularSystemState>) {
        if params.painting {
            params.paint();
        }
        egui::Window::new("Morph Parameters").show(contexts.ctx_mut(), |ui| {
            ui.add(
                egui::Slider::new(&mut params.carnivore_diffusion_factor, 0.0..=0.2)
                    .text("carnivore_diffusion_factor"),
            ); //: 0.03956,
            ui.add(
                egui::Slider::new(&mut params.plant_diffusion_factor, 0.0..=0.2)
                    .text("plant_diffusion_factor"),
            ); //: 0.01592,
            ui.add(
                egui::Slider::new(&mut params.herbivore_diffusion_factor, 0.0..=0.2)
                    .text("herbivore_diffusion_factor"),
            ); //: 0.02515,
            ui.add(
                egui::Slider::new(&mut params.survival_rate_carnivores, 0.98..=1.0)
                    .text("survival_rate_carnivores"),
            ); //: 0.995,
            ui.add(
                egui::Slider::new(&mut params.survival_rate_herbivores, 0.98..=1.0)
                    .text("survival_rate_herbivores"),
            ); //: 0.9995,
            ui.add(
                egui::Slider::new(&mut params.plant_growth_factor, 1.0..=1.1)
                    .text("plant_growth_factor"),
            ); //: 1.04,
            ui.add(
                egui::Slider::new(&mut params.herbivore_eating_ratio, 0.01..=0.3)
                    .text("herbivore_eating_ratio"),
            ); //: 0.06915,
            ui.add(
                egui::Slider::new(&mut params.carnivore_eating_ratio, 0.01..=0.3)
                    .text("carnivore_eating_ratio"),
            ); //: 0.09852391,
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button("Preset 1").clicked() {
                    params.carnivore_diffusion_factor = 0.03956;
                    params.plant_diffusion_factor = 0.01592;
                    params.herbivore_diffusion_factor = 0.02515;
                    params.survival_rate_carnivores = 0.995;
                    params.survival_rate_herbivores = 0.9995;
                    params.plant_growth_factor = 1.04;
                    params.herbivore_eating_ratio = 0.06915;
                    params.carnivore_eating_ratio = 0.09852391;
                };
                if ui.button("Preset 2").clicked() {
                    params.carnivore_diffusion_factor = 0.030;
                    params.plant_diffusion_factor = 0.050;
                    params.herbivore_diffusion_factor = 0.040;
                    params.survival_rate_carnivores = 0.998;
                    params.survival_rate_herbivores = 0.995;
                    params.plant_growth_factor = 1.05;
                    params.herbivore_eating_ratio = 0.150;
                    params.carnivore_eating_ratio = 0.2;
                };
                if ui.button("Preset 3").clicked() {
                    params.carnivore_diffusion_factor = 0.030;
                    params.plant_diffusion_factor = 0.040;
                    params.herbivore_diffusion_factor = 0.130;
                    params.survival_rate_carnivores = 0.997;
                    params.survival_rate_herbivores = 0.995;
                    params.plant_growth_factor = 1.055;
                    params.herbivore_eating_ratio = 0.150;
                    params.carnivore_eating_ratio = 0.2;
                };
                if ui.button("Preset 4").clicked() {
                    params.carnivore_diffusion_factor = 0.040;
                    params.plant_diffusion_factor = 0.035;
                    params.herbivore_diffusion_factor = 0.115;
                    params.survival_rate_carnivores = 0.9985;
                    params.survival_rate_herbivores = 0.998;
                    params.plant_growth_factor = 1.046;
                    params.herbivore_eating_ratio = 0.170;
                    params.carnivore_eating_ratio = 0.3;
                };
            });
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.heading("Paint");
                ui.color_edit_button_rgb(&mut params.paint_color);
                ui.add(egui::Slider::new(&mut params.paint_radius, 1..=100).text("Radius"));
            });

            let new_name = format!("{}", params.texture_handle);
            let new_image = params.new_texture.clone();
            let raw_size = params.canvas_size;

            let texture_handle_to_render = params.texture.get_or_insert_with(|| {
                ui.ctx()
                    .load_texture(new_name, new_image, Default::default())
            });

            if let Some(pos) = ui
                .image(
                    texture_handle_to_render,
                    bevy_egui::egui::Vec2::new(raw_size[0], raw_size[1]),
                )
                .hover_pos()
            {
                params.painting = true;
                params.paint_pos = pos;
            } else {
                params.painting = false;
            }

            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.checkbox(&mut params.iterating, "keep runnning");
                if ui.button("Reset").clicked() {
                    params.resetting = true;
                }
            });
            ui.add(egui::Slider::new(&mut params.render_channel, 0..=3).text("Render Channel"));
            egui::warn_if_debug_build(ui);
        });
    }

    pub fn setup_3d_scene(
        mut commands: Commands,
    ) {
        commands.spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 6.0, 4.0),
            ..default()
        });
        commands.spawn(Camera3dBundle {
            transform: Transform::from_xyz(-1.0, 3.5, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
    }

    pub fn update_3d_scene(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut mesh: ResMut<state::HeightMapMesh>,
        params: Res<state::CellularSystemState>,
    ) {
        let height_map = height_map::height_map_from_channel(params, 5.5);

        let mut height_mesh =
            Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);

        height_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, height_map.vertices);
        height_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, height_map.normals);
        height_mesh.set_indices(Some(bevy::render::mesh::Indices::U32(height_map.indices)));

        if let Some(id) = &mesh.mesh {
            meshes.remove(id);
        }
        let height_mesh_handle = meshes.add(height_mesh);
        mesh.mesh = Some(height_mesh_handle.clone());

        commands.spawn(PbrBundle {
            mesh: height_mesh_handle,
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });
    }
}
