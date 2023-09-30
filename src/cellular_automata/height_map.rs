use bevy::prelude::*;

use super::{state::CellularSystemState, torus_topology};

pub struct HeightMapMeshData {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub indices: Vec<u32>,
}

pub fn height_map_from_channel(params: Res<CellularSystemState>, size: f32) -> HeightMapMeshData {
    let vertices: Vec<Vec3> = params
        .new_texture
        .pixels
        .iter()
        .enumerate()
        .map(|(i, pixel)| {
            let height_value = match params.render_channel {
                0 => pixel.r(),
                1 => pixel.g(),
                2 => pixel.b(),
                _ => ((pixel.r() as f32 + pixel.g() as f32 + pixel.b() as f32) / 3.0) as u8,
            };
            [
                size * ((i % params.map_size[0]) as f32 / params.map_size[0] as f32 - 0.5),
                0.5 * height_value as f32 / 255.0,
                size * ((i / params.map_size[0]) as f32 / params.map_size[1] as f32 - 0.5),
            ]
            .into()
        })
        .collect();
    let indices = height_map_triangle_indices(params.map_size[0], params.map_size[1]);
    let normals = calculate_normals(&vertices, params.map_size[0], params.map_size[1]);

    HeightMapMeshData {
        vertices,
        normals,
        indices,
    }
}

fn height_map_triangle_indices(width: usize, height: usize) -> Vec<u32> {
    let mut indexlist: Vec<u32> = vec![];
    for x in 0..(width - 1) {
        for y in 0..(height - 1) {
            indexlist.push(((y * height) + x) as u32);

            indexlist.push(((y * height) + x + width) as u32);
            indexlist.push(((y * height) + x + 1) as u32);

            indexlist.push(((y * height) + x + 1) as u32);

            indexlist.push(((y * height) + x + width) as u32);
            indexlist.push(((y * height) + x + width + 1) as u32);
        }
    }
    indexlist
}

fn calculate_normals(vertices: &[Vec3], width: usize, height: usize) -> Vec<Vec3> {
    let mut normalslist: Vec<Vec3> = vec![Vec3::default(); width * height];

    for x in 0..(width as i32) {
        for y in 0..(height as i32) {
            let h_l =
                vertices[torus_topology::modulo_robust(x - 1, width as i32) + width * y as usize].y;
            let h_r =
                vertices[torus_topology::modulo_robust(x + 1, width as i32) + width * y as usize].y;
            let h_d = vertices
                [x as usize + width * torus_topology::modulo_robust(y - 1, height as i32)]
            .y;
            let h_u = vertices
                [x as usize + width * torus_topology::modulo_robust(y + 1, height as i32)]
            .y;
            normalslist[x as usize + width * y as usize] =
                Vec3::new(h_l - h_r, h_d - h_u, (4.0 * 5.5) / 160.0).normalize();
        }
    }
    normalslist
}
