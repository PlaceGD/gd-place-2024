use std::mem::offset_of;

use glam::Vec2;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Vertex {
    pub pos: Vec2,
}

impl Vertex {
    pub fn new(pos: Vec2) -> Self {
        Self { pos }
    }
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // wgpu::VertexAttribute {
                //     offset: const { offset_of!(Vertex, opacity) } as wgpu::BufferAddress,
                //     shader_location: 8,
                //     format: wgpu::VertexFormat::Float32,
                // },
                // wgpu::VertexAttribute {
                //     offset: const { offset_of!(Vertex, color) } as wgpu::BufferAddress,
                //     shader_location: 1,
                //     format: wgpu::VertexFormat::Float32x4,
                // },
                // wgpu::VertexAttribute {
                //     offset: const { offset_of!(Vertex, uv) } as wgpu::BufferAddress,
                //     shader_location: 2,
                //     format: wgpu::VertexFormat::Float32x2,
                // },
                // wgpu::VertexAttribute {
                //     offset: const { offset_of!(Vertex, tex_binding) } as wgpu::BufferAddress,
                //     shader_location: 3,
                //     format: wgpu::VertexFormat::Uint32,
                // },
            ],
        }
    }
}
