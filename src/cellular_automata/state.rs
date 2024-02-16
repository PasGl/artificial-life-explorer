use bevy::prelude::*;
use bevy_egui::egui;
use rand::prelude::Distribution;

use super::torus_topology;

#[derive(Resource)]
pub struct CellularSystemState {
    pub texture_handle: String,
    pub iterating: bool,
    pub diffusion_factor_red: f32,
    pub diffusion_factor_green: f32,
    pub diffusion_factor_blue: f32,
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
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
    pub g: f32,
    pub h: f32,
    pub i: f32,
    pub j: f32,
    pub k: f32,
    pub l: f32,
}

#[derive(Clone, Default, Resource)]
pub struct HeightMapMesh {
    pub mesh: Option<Handle<Mesh>>,
}

impl CellularSystemState {
    pub fn paint(&mut self) {
        let center_x = ((self.paint_pos.x * ((self.map_size[0] as f32) / self.canvas_size[0]))
            as i32)
            .clamp(0, (self.map_size[0] - 1) as i32);
        let center_y = ((self.paint_pos.y * ((self.map_size[1] as f32) / self.canvas_size[1]))
            as i32)
            .clamp(0, (self.map_size[1] - 1) as i32);
        let radius = self.paint_radius as i32;
        for r in 0..2 * radius {
            for s in 0..2 * radius {
                if (r - radius) * (r - radius) + (s - radius) * (s - radius) <= radius * radius {
                    self.new_texture[(
                        torus_topology::modulo_robust(
                            center_x + r - radius,
                            self.map_size[0] as i32,
                        ),
                        torus_topology::modulo_robust(
                            center_y + s - radius,
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
            diffusion_factor_red: 0.02248,
            diffusion_factor_green: 0.08454,
            diffusion_factor_blue: 0.05918,
            painting: false,
            paint_pos: [50.0, 50.0].into(),
            paint_color: [1.0, 1.0, 1.0],
            paint_radius: 20,
            resetting: false,
            render_channel: 3,
            map_size: [160, 160],
            canvas_size: [320.0, 320.0],
            a: 0.98204,
            b: 3.195,
            c: 0.112,
            d: 0.458,
            e: 0.92762,
            f: 4.666,
            g: 0.62,
            h: 0.981,
            i: 0.92696,
            j: 2.015,
            k: 0.465,
            l: 0.727,
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
                    let red = params.new_texture[(x, y)].to_array()[0] as f32;
                    let green = params.new_texture[(x, y)].to_array()[1] as f32;
                    let blue = params.new_texture[(x, y)].to_array()[2] as f32;

                    let pixel_r = diffusion(red, r_sum_neighbours, params.diffusion_factor_red)
                        + (255.0 * reaction_red(red / 255.0, green / 255.0, blue / 255.0, &params));

                    let pixel_g = diffusion(green, g_sum_neighbours, params.diffusion_factor_green)
                        + (255.0
                            * reaction_green(red / 255.0, green / 255.0, blue / 255.0, &params));

                    let pixel_b = diffusion(blue, b_sum_neighbours, params.diffusion_factor_blue)
                        + (255.0
                            * reaction_blue(red / 255.0, green / 255.0, blue / 255.0, &params));

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

fn diffusion(concentration: f32, weighted_sum_neighbors: f32, diffusion_coefficient: f32) -> f32 {
    concentration - 6.836879433 * concentration * diffusion_coefficient
        + weighted_sum_neighbors * diffusion_coefficient
}

fn reaction_red(
    red: f32,
    green: f32,
    blue: f32,
    params: &bevy::prelude::ResMut<'_, CellularSystemState>,
) -> f32 {
    //params[0] * red * (1.0 - red) - params[1] * red * green
    params.a * red * (1.0 - red) - ((params.b * red * green) / (1.0 + params.c * red * red))
        + params.d * (green - red) * blue * blue
}

fn reaction_green(
    red: f32,
    green: f32,
    blue: f32,
    params: &bevy::prelude::ResMut<'_, CellularSystemState>,
) -> f32 {
    //params[2] * green * (1.0 - green) - params[3] * green * blue
    params.e * green * (1.0 - green)
        - ((params.f * green * blue) / (1.0 + params.g * green * green))
        + params.h * (blue - green) * red * red
}

fn reaction_blue(
    red: f32,
    green: f32,
    blue: f32,
    params: &bevy::prelude::ResMut<'_, CellularSystemState>,
) -> f32 {
    //params[4] * blue * (1.0 - blue) - params[5] * blue * red
    params.i * blue * (1.0 - blue) - ((params.j * blue * red) / (1.0 + params.k * blue * blue))
        + params.l * (red - blue) * green * green
}
