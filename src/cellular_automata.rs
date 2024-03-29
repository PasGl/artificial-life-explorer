use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use rand::distributions::Distribution;
pub(crate) mod height_map;
pub(crate) mod state;
mod torus_topology;

pub fn egui_system(
    mut contexts: EguiContexts,
    mut params: ResMut<state::CellularSystemState>,
    mut timer: ResMut<Time<Fixed>>,
) {
    if params.painting {
        params.paint();
    }
    egui::Window::new("Control").show(contexts.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.checkbox(&mut params.iterating, "keep runnning at");
            let fps_slider = ui.add(egui::Slider::new(&mut params.fps, 1.0..=120.0).text("FPS"));
            if fps_slider.dragged() {
                timer.set_timestep_hz(params.fps);
            }
        });
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.button("Reset Map").clicked() {
                params.resetting = true;
            }
            if ui.button("Random Rules").clicked() {
                params.red = random_channel_parameters();
                params.green = random_channel_parameters();
                params.blue = random_channel_parameters();
            };
            if ui.button("Reset Map & Random Rules").clicked() {
                params.resetting = true;
                params.red = random_channel_parameters();
                params.green = random_channel_parameters();
                params.blue = random_channel_parameters();
            }
        });
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.heading("Heightmap");
            if ui
                .add(egui::RadioButton::new(params.render_channel == 0, "Red"))
                .clicked()
            {
                params.render_channel = 0;
            }
            if ui
                .add(egui::RadioButton::new(params.render_channel == 1, "Green"))
                .clicked()
            {
                params.render_channel = 1;
            }
            if ui
                .add(egui::RadioButton::new(params.render_channel == 2, "Blue"))
                .clicked()
            {
                params.render_channel = 2;
            }
            if ui
                .add(egui::RadioButton::new(params.render_channel == 3, "Sum"))
                .clicked()
            {
                params.render_channel = 3;
            }
        });
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.heading("Paint");
            ui.color_edit_button_rgb(&mut params.paint_color);
            ui.add(egui::Slider::new(&mut params.paint_radius, 1..=100).text("px Radius"));
        });
        egui::warn_if_debug_build(ui);

        let new_name = params.texture_handle.to_string();
        let new_image = params.new_texture.clone();
        let raw_size = params.canvas_size;

        let texture_handle_to_render = params.texture.get_or_insert_with(|| {
            ui.ctx()
                .load_texture(new_name, new_image, Default::default())
        });

        let img = ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
            texture_handle_to_render.id(),
            bevy_egui::egui::Vec2::new(raw_size[0], raw_size[1]),
        )));

        if let Some(pos) = img.hover_pos() {
            let min_pos = img.rect.min;
            params.painting = true;
            params.paint_pos = bevy_egui::egui::Pos2::new(pos.x - min_pos.x, pos.y - min_pos.y);
        } else {
            params.painting = false;
        }
    });
    egui::Window::new("Red").show(contexts.ctx_mut(), |ui| {
        add_channel_ui(&mut params.red, ui, "Red".to_owned());
    });
    egui::Window::new("Green").show(contexts.ctx_mut(), |ui| {
        add_channel_ui(&mut params.green, ui, "Green".to_owned());
    });
    egui::Window::new("Blue").show(contexts.ctx_mut(), |ui| {
        add_channel_ui(&mut params.blue, ui, "Blue".to_owned());
    });
}

fn add_channel_ui(channel: &mut state::ChannelParameters, ui: &mut egui::Ui, label: String) {
    ui.add(
        egui::Slider::new(&mut channel.diffusion_coefficient, 0.0..=0.1)
            .text(format!("{} Diffusion", label)),
    );
    ui.add(
        egui::Slider::new(&mut channel.growth_rate, 0.7..=1.0).text(format!("{} Growth", label)),
    );
    ui.add(
        egui::Slider::new(&mut channel.interaction_coefficient, 0.0..=5.0)
            .text(format!("{} Interaction", label)),
    );
    ui.add(
        egui::Slider::new(&mut channel.saturation_constant, 0.0..=3.0)
            .text(format!("{} Saturation", label)),
    );
    ui.add(
        egui::Slider::new(&mut channel.feedback_coefficient, 0.0..=3.0)
            .text(format!("{} Feedback", label)),
    );
}

fn random_channel_parameters() -> state::ChannelParameters {
    let mut rng = rand::thread_rng();
    let p0 = rand::distributions::Uniform::new_inclusive(0.0, 0.1);
    let p1 = rand::distributions::Uniform::new_inclusive(0.7, 1.0);
    let p2 = rand::distributions::Uniform::new_inclusive(0.0, 5.0);
    let p3 = rand::distributions::Uniform::new_inclusive(0.0, 3.0);
    let p4 = rand::distributions::Uniform::new_inclusive(0.0, 3.0);
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
