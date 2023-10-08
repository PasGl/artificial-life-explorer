use bevy::prelude::*;
use bevy_egui::egui;
use rand::prelude::Distribution;

use super::torus_topology;

#[derive(Resource)]
pub struct CellularSystemState {
    pub texture_handle: String,
    pub iterating: bool,
    pub carnivore_diffusion_factor: f32,
    pub plant_diffusion_factor: f32,
    pub herbivore_diffusion_factor: f32,
    pub survival_rate_carnivores: f32,
    pub survival_rate_herbivores: f32,
    pub plant_growth_factor: f32,
    pub herbivore_eating_ratio: f32,
    pub carnivore_eating_ratio: f32,
    iteration_in_buffer: u64,
    iterations_done: u64,
    pub texture: Option<egui::TextureHandle>,
    pub new_texture: egui::ColorImage,
    pub painting: bool,
    pub paint_pos: egui::Pos2,
    pub paint_color: [f32; 3],
    pub paint_radius: usize,
    pub resetting: bool,
    pub render_channel: usize,
    pub map_size: [usize; 2],
    pub canvas_size: [f32; 2],
}

#[derive(Clone, Default, Resource)]
pub struct HeightMapMesh {
    pub mesh: Option<Handle<Mesh>>,
}

impl CellularSystemState {
    pub fn paint(&mut self) {
        let center_x = (((self.paint_pos.x - 23.0)
            * ((self.map_size[0] as f32) / self.canvas_size[0])) as i32)
            .clamp(0, (self.map_size[0] - 1) as i32);
        let center_y = (((self.paint_pos.y - 265.0)
            * ((self.map_size[1] as f32) / self.canvas_size[1])) as i32)
            .clamp(0, (self.map_size[1] - 1) as i32);
        for r in 0..(2 * self.paint_radius as i32) {
            for s in 0..(2 * self.paint_radius as i32) {
                let radius = self.paint_radius as i32;
                if (r - radius) * (r - radius)
                    + (s - radius) * (s - radius)
                    <= radius * radius
                {
                    self.new_texture[(
                        torus_topology::modulo_robust(
                            (center_x + r) as i32 - self.paint_radius as i32,
                            self.map_size[0] as i32,
                        ),
                        torus_topology::modulo_robust(
                            (center_y + s) as i32 - self.paint_radius as i32,
                            self.map_size[1] as i32,
                        ),
                    )] = egui::Color32::from_rgb(
                        (self.paint_color[0] * 255.0) as u8,
                        (self.paint_color[1] * 255.0) as u8,
                        (self.paint_color[2] * 255.0) as u8,
                    );
                }
            }
        }
    }
}

impl Default for CellularSystemState {
    fn default() -> Self {
        Self {
            texture_handle: "0".to_owned(),
            iteration_in_buffer: 0,
            iterations_done: 0,
            texture: None,
            new_texture: initial_system(160, 160),
            iterating: true,
            carnivore_diffusion_factor: 0.03956,
            plant_diffusion_factor: 0.01592,
            herbivore_diffusion_factor: 0.02515,
            survival_rate_carnivores: 0.995,
            survival_rate_herbivores: 0.9995,
            plant_growth_factor: 1.04,
            herbivore_eating_ratio: 0.06915,
            carnivore_eating_ratio: 0.09852391,
            painting: false,
            paint_pos: [50.0, 50.0].into(),
            paint_color: [1.0, 0.0, 0.0],
            paint_radius: 20,
            resetting: false,
            render_channel: 0,
            map_size: [160, 160],
            canvas_size: [320.0, 320.0],
        }
    }
}

fn initial_system(width: usize, height: usize) -> egui::ColorImage {
    let mut rng = rand::thread_rng();
    let p = rand::distributions::Uniform::new_inclusive(0.0, 1.0);

    let mut current_image =
        egui::ColorImage::new([width, height], egui::Color32::from_rgb(128, 0, 50));
    for x in 0..width {
        for y in 0..height {
            current_image[(x, y)] = egui::Color32::from_rgb(
                (255.0 * p.sample(&mut rng) * (y as f32 / (height as f32))) as u8,
                (255.0
                    * p.sample(&mut rng)
                    * ((x as f32) * (y as f32) / ((height as f32) * (width as f32))))
                    as u8,
                (255.0 * p.sample(&mut rng) * (x as f32 / (width as f32))) as u8,
            );
        }
    }
    current_image
}

pub fn next_iteration(mut params: ResMut<CellularSystemState>) {
    params.iteration_in_buffer += 1;
    if (params.iteration_in_buffer > params.iterations_done && params.iterating)
        || params.resetting
        || params.painting
    {
        params.iterations_done += 1;
        let mut new_image = egui::ColorImage::new(
            [params.map_size[0], params.map_size[1]],
            egui::Color32::from_rgb(
                (params.iteration_in_buffer / 10) as u8,
                params.iteration_in_buffer as u8,
                50,
            ),
        );
        if params.resetting {
            new_image = initial_system(params.map_size[0], params.map_size[1]);
            params.resetting = false;
        } else {
            for x in 0..params.map_size[0] {
                for y in 0..params.map_size[1] {
                    let r_sum_neighbours = torus_topology::sum_neighbour_channel(
                        &params.new_texture,
                        x as i32,
                        y as i32,
                        params.map_size[0] as i32,
                        params.map_size[1] as i32,
                        0,
                    );

                    let g_sum_neighbours = torus_topology::sum_neighbour_channel(
                        &params.new_texture,
                        x as i32,
                        y as i32,
                        params.map_size[0] as i32,
                        params.map_size[1] as i32,
                        1,
                    );
                    let b_sum_neighbours = torus_topology::sum_neighbour_channel(
                        &params.new_texture,
                        x as i32,
                        y as i32,
                        params.map_size[0] as i32,
                        params.map_size[1] as i32,
                        2,
                    );
                    let current_cell_carnivores = params.new_texture[(x, y)].to_array()[0] as f32;
                    let current_cell_plants = params.new_texture[(x, y)].to_array()[1] as f32;
                    let current_cell_herbivores = params.new_texture[(x, y)].to_array()[2] as f32;

                    let pixel_r = // survivers in this cell
            (current_cell_carnivores * params.survival_rate_carnivores)
            // + migration from neighbors
            + (r_sum_neighbours * params.carnivore_diffusion_factor)
            // - migration to neighbors
            - (current_cell_carnivores * 8.0 * params.carnivore_diffusion_factor)
            // + growth from eating herbivores
            + (current_cell_herbivores * (current_cell_carnivores/255.0) * params.carnivore_eating_ratio);

                    let pixel_g = // growth from sun
            (current_cell_plants * params.plant_growth_factor)
            // + migration from neighbors
            + (g_sum_neighbours * params.plant_diffusion_factor)
            // - migration to neighbors
            - (current_cell_plants * 8.0 * params.plant_diffusion_factor)
            // - eaten by herbivores
            - (current_cell_herbivores * (current_cell_plants/255.0)* params.herbivore_eating_ratio);

                    let pixel_b = // survivers in this cell
            (current_cell_herbivores * params.survival_rate_herbivores)
            // +migration from neighbors
            + (b_sum_neighbours * params.herbivore_diffusion_factor)
            // - migration to neighbors
            - (current_cell_herbivores * 8.0 *  params.herbivore_diffusion_factor)
            // + growth from eating plants
            + (current_cell_plants * (current_cell_herbivores/255.0) * params.herbivore_eating_ratio)
            // - eaten by carnivores
            - (current_cell_carnivores * (current_cell_herbivores/255.0) * params.carnivore_eating_ratio);

                    new_image[(x, y)] =
                        egui::Color32::from_rgb(pixel_r as u8, pixel_g as u8, pixel_b as u8);
                }
            }
        }

        let strg = params.iteration_in_buffer.to_string();
        params.new_texture = new_image;
        params.texture_handle = strg;
        let t: Option<egui::TextureHandle> = None;
        params.texture = t;
    }
}
