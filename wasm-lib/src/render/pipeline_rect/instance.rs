use std::mem::offset_of;

use glam::{Affine2, Mat2, Vec2, Vec4};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Instance {
    pub pos: [f32; 2],
    pub t_x: [f32; 2],
    pub t_y: [f32; 2],
    pub color: [f32; 4],
    // 0
    // 1, 2, 3
    // 101,
    pub img: u32,
    pub uv_pos: [f32; 2],
    pub uv_size: [f32; 2],
}

impl Instance {
    pub fn new(
        affine: Affine2,
        // pos: Vec2,
        // transform: Mat2,
        color: Vec4,
        img: u32,
        uv_pos: Vec2,
        uv_size: Vec2,
    ) -> Self {
        Self {
            pos: affine.translation.to_array(),
            t_x: affine.matrix2.x_axis.to_array(),
            t_y: affine.matrix2.y_axis.to_array(),
            color: color.to_array(),
            uv_pos: uv_pos.to_array(),
            uv_size: uv_size.to_array(),
            img,
        }
    }
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: const { offset_of!(Instance, t_x) } as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: const { offset_of!(Instance, t_y) } as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: const { offset_of!(Instance, color) } as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: const { offset_of!(Instance, img) } as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Uint32,
                },
                wgpu::VertexAttribute {
                    offset: const { offset_of!(Instance, uv_pos) } as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: const { offset_of!(Instance, uv_size) } as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}
