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
            egui::Slider::new(&mut params.diffusion_factor_red, 0.001..=0.1)
                .text("diffusion_factor_red"),
        ); //: 0.03956,
        ui.add(
            egui::Slider::new(&mut params.diffusion_factor_green, 0.001..=0.1)
                .text("diffusion_factor_green"),
        ); //: 0.01592,
        ui.add(
            egui::Slider::new(&mut params.diffusion_factor_blue, 0.001..=0.1)
                .text("diffusion_factor_blue"),
        ); //: 0.02515,
        ui.add(egui::Slider::new(&mut params.a, 0.9..=0.99).text("a"));
        ui.add(egui::Slider::new(&mut params.b, 0.01..=10.0).text("b"));
        ui.add(egui::Slider::new(&mut params.c, 0.1..=100.0).text("c"));
        ui.add(egui::Slider::new(&mut params.d, 0.001..=10.0).text("d"));
        ui.add(egui::Slider::new(&mut params.e, 0.9..=0.99).text("e"));
        ui.add(egui::Slider::new(&mut params.f, 0.01..=10.0).text("f"));
        ui.add(egui::Slider::new(&mut params.g, 0.1..=100.0).text("g"));
        ui.add(egui::Slider::new(&mut params.h, 0.001..=10.0).text("h"));
        ui.add(egui::Slider::new(&mut params.i, 0.9..=0.99).text("i"));
        ui.add(egui::Slider::new(&mut params.j, 0.01..=10.0).text("j"));
        ui.add(egui::Slider::new(&mut params.k, 0.1..=100.0).text("k"));
        ui.add(egui::Slider::new(&mut params.l, 0.001..=10.0).text("l"));

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.checkbox(&mut params.iterating, "keep runnning");
            if ui.button("Reset").clicked() {
                params.resetting = true;
            }
            if ui.button("Random").clicked() {
                let mut rng = rand::thread_rng();
                let p0 = rand::distributions::Uniform::new_inclusive(0.001, 0.1);
                let p1 = rand::distributions::Uniform::new_inclusive(0.9, 0.99);
                let p2 = rand::distributions::Uniform::new_inclusive(0.4, 5.0);
                let p3 = rand::distributions::Uniform::new_inclusive(0.1, 1.0);
                let p4 = rand::distributions::Uniform::new_inclusive(0.001, 1.0);
                params.diffusion_factor_red = p0.sample(&mut rng);
                params.diffusion_factor_green = p0.sample(&mut rng);
                params.diffusion_factor_blue = p0.sample(&mut rng);
                params.a = p1.sample(&mut rng);
                params.b = p2.sample(&mut rng);
                params.c = p3.sample(&mut rng);
                params.d = p4.sample(&mut rng);
                params.e = p1.sample(&mut rng);
                params.f = p2.sample(&mut rng);
                params.g = p3.sample(&mut rng);
                params.h = p4.sample(&mut rng);
                params.i = p1.sample(&mut rng);
                params.j = p2.sample(&mut rng);
                params.k = p3.sample(&mut rng);
                params.l = p4.sample(&mut rng);
            };
            if ui.button("Reset & Random").clicked() {
                params.resetting = true;
                let mut rng = rand::thread_rng();
                let p0 = rand::distributions::Uniform::new_inclusive(0.001, 0.1);
                let p1 = rand::distributions::Uniform::new_inclusive(0.9, 0.99);
                let p2 = rand::distributions::Uniform::new_inclusive(0.4, 5.0);
                let p3 = rand::distributions::Uniform::new_inclusive(0.1, 1.0);
                let p4 = rand::distributions::Uniform::new_inclusive(0.001, 1.0);
                params.diffusion_factor_red = p0.sample(&mut rng);
                params.diffusion_factor_green = p0.sample(&mut rng);
                params.diffusion_factor_blue = p0.sample(&mut rng);
                params.a = p1.sample(&mut rng);
                params.b = p2.sample(&mut rng);
                params.c = p3.sample(&mut rng);
                params.d = p4.sample(&mut rng);
                params.e = p1.sample(&mut rng);
                params.f = p2.sample(&mut rng);
                params.g = p3.sample(&mut rng);
                params.h = p4.sample(&mut rng);
                params.i = p1.sample(&mut rng);
                params.j = p2.sample(&mut rng);
                params.k = p3.sample(&mut rng);
                params.l = p4.sample(&mut rng);
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
