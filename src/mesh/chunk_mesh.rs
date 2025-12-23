use glam::{Mat4, Vec3};
use wgpu::Device;

use crate::{
    graphics::Renderable,
    mesh::{InstanceData, InstanceMesh, TileMesh, VertexData},
};

pub struct ChunkMesh {
    pub instance_mesh: InstanceMesh,
}

impl ChunkMesh {
    pub fn new(device: &Device, pos: [i64; 2], size: u8, scale: f32) -> Self {
        let mut vertices: Vec<VertexData> = vec![];
        let mesh_data = TileMesh::to_mesh_data();
        vertices.extend_from_slice(&mesh_data.vertices);
        let indices = if let Some(indices) = mesh_data.indices {
            Some(indices)
        } else {
            None
        };

        let size_i64 = size as i64;

        let transform_scale = Mat4::from_scale(Vec3::new(scale, scale, 0.0));

        let mut instances: Vec<InstanceData> = vec![];

        for x in (pos[0] - size_i64)..=(pos[0] + size_i64) {
            for y in (pos[1] - size_i64)..=(pos[1] + size_i64) {
                let model = Mat4::from_translation(
                    scale
                        * 2.0
                        * Vec3 {
                            x: x as f32,
                            y: y as f32,
                            z: 0.0,
                        },
                ) * transform_scale;
                let data = InstanceData::new(
                    model,
                    [(x * 10) as u8, (y * 10) as u8, ((x + y) * 10) as u8, 255],
                );
                instances.push(data);
            }
        }

        let instance_mesh = InstanceMesh::new(device, &vertices, indices, &instances);

        Self {
            instance_mesh: instance_mesh,
        }
    }
}

impl Renderable for ChunkMesh {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.instance_mesh.render(render_pass);
    }
}
