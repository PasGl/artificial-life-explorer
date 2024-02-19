use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use rand::distributions::Distribution;
pub(crate) mod height_map;
pub(crate) mod state;
mod torus_topology;

pub fn egui_system(mut contexts: EguiContexts, mut params: ResMut<state::CellularSystemState>) {
    if params.painting {
        params.paint();
    }
    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        ui.add(
            egui::Slider::new(&mut params.red.diffusion_coefficient, 0.0..=0.1)
                .text("Red Diffusion"),
        );
        ui.add(egui::Slider::new(&mut params.red.growth_rate, 0.8..=1.0).text("Red Growth"));
        ui.add(
            egui::Slider::new(&mut params.red.interaction_coefficient, 0.0..=5.0)
                .text("Red Interaction"),
        );
        ui.add(
            egui::Slider::new(&mut params.red.saturation_constant, 0.0..=5.0)
                .text("Red Saturation"),
        );
        ui.add(
            egui::Slider::new(&mut params.red.feedback_coefficient, 0.0..=5.0).text("Red Feedback"),
        );
        ui.add(
            egui::Slider::new(&mut params.green.diffusion_coefficient, 0.0..=0.1)
                .text("Green Diffusion"),
        );
        ui.add(egui::Slider::new(&mut params.green.growth_rate, 0.8..=5.0).text("Green Growth"));
        ui.add(
            egui::Slider::new(&mut params.green.interaction_coefficient, 0.0..=5.0)
                .text("Green Interaction"),
        );
        ui.add(
            egui::Slider::new(&mut params.green.saturation_constant, 0.0..=5.0)
                .text("Green Saturation"),
        );
        ui.add(
            egui::Slider::new(&mut params.green.feedback_coefficient, 0.0..=5.0)
                .text("Green Feedback"),
        );
        ui.add(
            egui::Slider::new(&mut params.blue.diffusion_coefficient, 0.0..=0.1)
                .text("Blue Diffusion"),
        );
        ui.add(egui::Slider::new(&mut params.blue.growth_rate, 0.8..=1.0).text("Blue Growth"));
        ui.add(
            egui::Slider::new(&mut params.blue.interaction_coefficient, 0.0..=5.0)
                .text("Blue Interaction"),
        );
        ui.add(
            egui::Slider::new(&mut params.blue.saturation_constant, 0.0..=5.0)
                .text("Blue Saturation"),
        );
        ui.add(
            egui::Slider::new(&mut params.blue.feedback_coefficient, 0.0..=5.0)
                .text("Blue Feedback"),
        );

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.checkbox(&mut params.iterating, "keep runnning");
            if ui.button("Reset").clicked() {
                params.resetting = true;
            }
            if ui.button("Random").clicked() {
                params.red = random_channel_parameters();
                params.green = random_channel_parameters();
                params.blue = random_channel_parameters();
            };
            if ui.button("Reset & Random").clicked() {
                params.resetting = true;
                params.red = random_channel_parameters();
                params.green = random_channel_parameters();
                params.blue = random_channel_parameters();
            }
        });
        ui.add(egui::Slider::new(&mut params.render_channel, 0..=3).text("Render Channel"));
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.heading("Paint");
            ui.color_edit_button_rgb(&mut params.paint_color);
            ui.add(egui::Slider::new(&mut params.paint_radius, 1..=100).text("Radius"));
        });
        egui::warn_if_debug_build(ui);

        let new_name = params.texture_handle.to_string();
        let new_image = params.new_texture.clone();
        let raw_size = params.canvas_size;

        let texture_handle_to_render = params.texture.get_or_insert_with(|| {
            ui.ctx()
                .load_texture(new_name, new_image, Default::default())
        });

        let img = ui.image(
            texture_handle_to_render,
            bevy_egui::egui::Vec2::new(raw_size[0], raw_size[1]),
        );

        if let Some(pos) = img.hover_pos() {
            let min_pos = img.rect.min;
            params.painting = true;
            params.paint_pos = bevy_egui::egui::Pos2::new(pos.x - min_pos.x, pos.y - min_pos.y);
        } else {
            params.painting = false;
        }
    });
}

pub fn random_channel_parameters() -> state::ChannelParameters {
    let mut rng = rand::thread_rng();
    let p0 = rand::distributions::Uniform::new_inclusive(0.0, 0.1);
    let p1 = rand::distributions::Uniform::new_inclusive(0.8, 1.0);
    let p2 = rand::distributions::Uniform::new_inclusive(0.0, 5.0);
    let p3 = rand::distributions::Uniform::new_inclusive(0.0, 1.0);
    let p4 = rand::distributions::Uniform::new_inclusive(0.0, 1.0);
    state::ChannelParameters {
        diffusion_coefficient: p0.sample(&mut rng),
        growth_rate: p1.sample(&mut rng),
        interaction_coefficient: p2.sample(&mut rng),
        saturation_constant: p3.sample(&mut rng),
        feedback_coefficient: p4.sample(&mut rng),
    }
}

pub fn setup_3d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut mesh: ResMut<state::HeightMapMesh>,
    params: Res<state::CellularSystemState>,
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
    let height_map = height_map::height_map(params, 5.5);

    let mut height_mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);

    height_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, height_map.vertices);
    height_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, height_map.normals);
    height_mesh.set_indices(Some(bevy::render::mesh::Indices::U32(height_map.indices)));

    let height_mesh_handle = meshes.add(height_mesh);
    mesh.mesh = Some(height_mesh_handle.clone());

    commands.spawn(PbrBundle {
        mesh: height_mesh_handle,
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
