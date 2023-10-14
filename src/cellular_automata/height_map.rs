use bevy::prelude::*;

use super::{state::CellularSystemState, torus_topology};

pub struct HeightMapMeshData {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub indices: Vec<u32>,
}

pub fn height_map(params: Res<CellularSystemState>, size: f32) -> HeightMapMeshData {
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
                size * ((i % params.map_size[0]) as f32 / (params.map_size[0] - 1) as f32 - 0.5),
                0.5 * height_value as f32 / 255.0,
                size * (params.map_size[1] as f32 / params.map_size[0] as f32)
                    * ((i / params.map_size[0]) as f32 / (params.map_size[1] - 1) as f32 - 0.5),
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
            indexlist.push(((y * width) + x) as u32);
            indexlist.push(((y * width) + x + width) as u32);
            indexlist.push(((y * width) + x + 1) as u32);

            indexlist.push(((y * width) + x + 1) as u32);
            indexlist.push(((y * width) + x + width) as u32);
            indexlist.push(((y * width) + x + width + 1) as u32);
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

pub fn update_heightmap(
    mut meshes: ResMut<Assets<Mesh>>,
    mesh: ResMut<super::state::HeightMapMesh>,
    params: Res<super::state::CellularSystemState>,
) {
    if let Some(id) = &mesh.mesh {
        let active_mesh = meshes.get_mut(id).unwrap();
        let positions = active_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
        if let bevy::render::mesh::VertexAttributeValues::Float32x3(vertices) = positions {
            let new_vertices: Vec<Vec3> = vertices
                .iter()
                .enumerate()
                .map(|(i, pos)| {
                    let pixel = params.new_texture.pixels[i];
                    let height_value = match params.render_channel {
                        0 => pixel.r(),
                        1 => pixel.g(),
                        2 => pixel.b(),
                        _ => ((pixel.r() as f32 + pixel.g() as f32 + pixel.b() as f32) / 3.0) as u8,
                    };
                    [pos[0], 0.5 * height_value as f32 / 255.0, pos[2]].into()
                })
                .collect();
            let new_normals =
                calculate_normals(&new_vertices, params.map_size[0], params.map_size[1]);
            active_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_vertices);
            active_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, new_normals);
        }
    }
}
