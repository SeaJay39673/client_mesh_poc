use std::collections::HashMap;

use wgpu::Device;

use crate::{graphics::Renderable, mesh::ChunkMesh};

pub struct ChunkMeshes {
    meshes: HashMap<[i64; 2], ChunkMesh>,
}

impl ChunkMeshes {
    pub fn new(device: &Device, size: u8, scale: f32) -> Self {
        let mut meshes: HashMap<[i64; 2], ChunkMesh> = HashMap::new();

        let size_i64 = size as i64;

        for x in -size_i64..=size_i64 {
            for y in -size_i64..=size_i64 {
                let pos = [x as i64, y as i64];
                meshes.insert(pos, ChunkMesh::new(device, pos, size, scale));
            }
        }

        Self { meshes }
    }
}

impl Renderable for ChunkMeshes {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.meshes
            .values()
            .for_each(|chunk_mesh| chunk_mesh.render(render_pass));
    }
}
