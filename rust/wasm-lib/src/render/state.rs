use std::{error::Error, fmt::Display, fs, io, sync::Arc, time::Instant};

use glam::{uvec2, vec2, vec4, UVec2};
use image::{ImageError, ImageResult};
use wgpu::{util::DeviceExt, Buffer, CreateSurfaceError, WindowHandle};
use winit::dpi::PhysicalSize;

use crate::{
    config::Config,
    render::{data::Globals, pipeline_grid, pipeline_rect},
};

use super::{
    texture::Texture,
    util::{create_buffer_bind_group, create_pipeline},
};

#[derive(Debug)]
pub enum StateError {
    ImageError(ImageError),
    ImageReadError(io::Error),
    CreateSurfaceError(CreateSurfaceError),
}

impl Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateError::ImageError(e) => write!(f, "{e}"),
            StateError::ImageReadError(e) => write!(f, "failed to read image: {e}"),
            StateError::CreateSurfaceError(e) => write!(f, "{e}"),
        }
    }
}

impl Error for StateError {}

pub struct RenderState {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,

    pub globals_buffer: wgpu::Buffer,
    pub globals_bind_group: wgpu::BindGroup,

    pub textures_bind_group: wgpu::BindGroup,

    // pub onion_linear_bind_group: wgpu::BindGroup,
    // pub onion_nearest_bind_group: wgpu::BindGroup,
    // pub onion_size: UVec2,
    pub multisampled_frame_descriptor: wgpu::TextureDescriptor<'static>,

    pub pipeline_rect: wgpu::RenderPipeline,
    pub pipeline_rect_additive_sq_alpha: wgpu::RenderPipeline,
    pub pipeline_grid: wgpu::RenderPipeline,
    pub rect_vertex_buffer: Buffer,
    pub rect_index_buffer: Buffer,
}

fn create_textures_bind_group(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    config: &Config,
) -> Result<(wgpu::BindGroupLayout, wgpu::BindGroup), StateError> {
    // TODO: speed up this?
    let spritesheet =
        image::load_from_memory(include_bytes!("../../../../src/assets/spritesheet.png"))
            .map_err(StateError::ImageError)?;

    // TODO: better errors?
    let bg_data = fs::read(&config.background.image).map_err(StateError::ImageReadError)?;
    let bg = image::load_from_memory(&bg_data).map_err(StateError::ImageError)?;

    // let ground = image::load_from_memory(include_bytes!("../../assets/ground.png"))
    //     .map_err(StateError::ImageError)?;
    // let spritesheet = image::load_from_memory(spritesheet_data).unwrap();

    // let list = [
    //     (&bg, false),
    //     (&ground, false),
    //     (&spritesheet, false),
    //     (&spritesheet, true),
    // ];

    let textures_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                // wgpu::BindGroupLayoutEntry {
                //     binding: 2,
                //     visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                //     ty: wgpu::BindingType::Texture {
                //         multisampled: false,
                //         view_dimension: wgpu::TextureViewDimension::D2,
                //         sample_type: wgpu::TextureSampleType::Float { filterable: true },
                //     },
                //     count: None,
                // },
                // wgpu::BindGroupLayoutEntry {
                //     binding: 3,
                //     visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                //     ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                //     count: None,
                // },

                // spritesheet nearest neighbour + linear
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // background
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: None,
        });

    let textures = vec![
        // Texture::from_image(device, queue, &ground, wgpu::FilterMode::Linear),
        Texture::from_image(device, queue, &spritesheet, wgpu::FilterMode::Linear),
        Texture::from_image(device, queue, &spritesheet, wgpu::FilterMode::Nearest),
        Texture::from_image(device, queue, &bg, wgpu::FilterMode::Linear),
    ];

    let mut entries = vec![];

    for (i, t) in textures.iter().enumerate() {
        entries.extend([
            wgpu::BindGroupEntry {
                binding: 2 * i as u32,
                resource: wgpu::BindingResource::TextureView(&t.view),
            },
            wgpu::BindGroupEntry {
                binding: 2 * i as u32 + 1,
                resource: wgpu::BindingResource::Sampler(&t.sampler),
            },
        ]);
    }

    let textures_bind_group = {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &textures_bind_group_layout,
            entries: &entries,
            label: None,
        })
    };

    Ok((textures_bind_group_layout, textures_bind_group))
}

pub const SAMPLE_COUNT: u32 = 4;
impl RenderState {
    pub async fn new(
        surface: wgpu::Surface<'static>,
        size: UVec2,
        instance: wgpu::Instance,
        config: &Config,
    ) -> Result<Self, StateError> {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::None,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| !f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.x,
            height: size.y,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        let (globals_buffer, globals_bind_group_layout, globals_bind_group) =
            create_buffer_bind_group(
                &device,
                std::mem::size_of::<Globals>() as u64,
                wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                wgpu::BufferBindingType::Uniform,
            );

        let (textures_bind_group_layout, textures_bind_group) =
            create_textures_bind_group(&device, &queue, config)?;

        let multisampled_frame_descriptor = wgpu::TextureDescriptor {
            label: Some("Multisampled frame descriptor"),
            size: wgpu::Extent3d {
                width: surface_config.width,
                height: surface_config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: SAMPLE_COUNT,
            dimension: wgpu::TextureDimension::D2,
            format: surface_config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        };
        let pipeline_rect = create_pipeline(
            &device,
            &device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("shader"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("./pipeline_rect/shader.wgsl").into(),
                ),
            }),
            &[&globals_bind_group_layout, &textures_bind_group_layout],
            &[
                pipeline_rect::vertex::Vertex::desc(),
                pipeline_rect::instance::Instance::desc(),
            ],
            &[Some(wgpu::ColorTargetState {
                format: surface_config.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: wgpu::BlendComponent::OVER,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            "vs_main",
            "fs_main",
        );
        let pipeline_rect_additive_sq_alpha = create_pipeline(
            &device,
            &device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("shader"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("./pipeline_rect/shader.wgsl").into(),
                ),
            }),
            &[&globals_bind_group_layout, &textures_bind_group_layout],
            &[
                pipeline_rect::vertex::Vertex::desc(),
                pipeline_rect::instance::Instance::desc(),
            ],
            &[Some(wgpu::ColorTargetState {
                format: surface_config.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::One,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: wgpu::BlendComponent::OVER,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            "vs_main",
            "fs_main_sq_alpha",
        );
        let pipeline_grid = create_pipeline(
            &device,
            &device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("shader"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("./pipeline_grid/shader.wgsl").into(),
                ),
            }),
            &[&globals_bind_group_layout, &textures_bind_group_layout],
            &[pipeline_grid::vertex::Vertex::desc()],
            &[Some(wgpu::ColorTargetState {
                format: surface_config.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: wgpu::BlendComponent::OVER,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            "vs_main",
            "fs_main",
        );

        let rect_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[
                pipeline_rect::vertex::Vertex::new(vec2(0.0, 0.0)),
                pipeline_rect::vertex::Vertex::new(vec2(1.0, 0.0)),
                pipeline_rect::vertex::Vertex::new(vec2(1.0, 1.0)),
                pipeline_rect::vertex::Vertex::new(vec2(0.0, 1.0)),
            ]),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let rect_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index_buffer"),
            contents: bytemuck::cast_slice::<u32, _>(&[0, 1, 2, 2, 3, 0]),
            usage: wgpu::BufferUsages::INDEX,
        });

        // let screen_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: None,
        //     contents: bytemuck::cast_slice(&[
        //         pipeline_rect::vertex::Vertex::new(vec2(0.0, 0.0)),
        //         pipeline_rect::vertex::Vertex::new(vec2(1.0, 0.0)),
        //         pipeline_rect::vertex::Vertex::new(vec2(1.0, 1.0)),
        //         pipeline_rect::vertex::Vertex::new(vec2(0.0, 1.0)),
        //     ]),
        //     usage: wgpu::BufferUsages::VERTEX,
        // });

        Ok(Self {
            surface,
            device,
            queue,
            surface_config,
            globals_buffer,
            globals_bind_group,
            textures_bind_group,

            multisampled_frame_descriptor,
            pipeline_rect,
            pipeline_rect_additive_sq_alpha,
            pipeline_grid,
            rect_vertex_buffer,
            rect_index_buffer,
        })
    }

    pub async fn new_canvas(
        window: impl WindowHandle + 'static,
        size: PhysicalSize<u32>,
        config: &Config,
    ) -> Result<Self, StateError> {
        // let size = uvec2(window.window_handle()/, window.height());

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance
            .create_surface(wgpu::SurfaceTarget::Window(Box::new(window)))
            .map_err(StateError::CreateSurfaceError)?;

        Self::new(
            surface,
            uvec2(size.width, size.height), // TODO: first time size?
            instance,
            config,
        )
        .await
    }

    pub fn resize(&mut self, width: u32, height: u32, quality: f32) {
        if width > 0 && height > 0 {
            let width = (width.min(4095) as f32 * quality).round() as u32;
            let height = (height.min(4095) as f32 * quality).round() as u32;
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);

            self.multisampled_frame_descriptor = wgpu::TextureDescriptor {
                label: Some("Multisampled frame descriptor"),
                size: wgpu::Extent3d {
                    width: self.surface_config.width,
                    height: self.surface_config.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: SAMPLE_COUNT,
                dimension: wgpu::TextureDimension::D2,
                format: self.surface_config.format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            };
        }
    }
}
