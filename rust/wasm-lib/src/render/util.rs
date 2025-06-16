use super::state::SAMPLE_COUNT;

pub fn create_buffer_bind_group(
    device: &wgpu::Device,
    buffer_byte_size: u64,
    buffer_usage: wgpu::BufferUsages,
    bind_group_visibility: wgpu::ShaderStages,
    buffer_binding_type: wgpu::BufferBindingType,
) -> (wgpu::Buffer, wgpu::BindGroupLayout, wgpu::BindGroup) {
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: buffer_byte_size,
        usage: buffer_usage,
        mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: bind_group_visibility,
            ty: wgpu::BindingType::Buffer {
                ty: buffer_binding_type,
                has_dynamic_offset: false,
                min_binding_size: wgpu::BufferSize::new(buffer_byte_size),
            },
            count: None,
        }],
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(buffer.as_entire_buffer_binding()),
        }],
    });

    (buffer, bind_group_layout, bind_group)
}

pub fn create_pipeline(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
    vertex_buffer_layouts: &[wgpu::VertexBufferLayout<'_>],
    fragment_targets: &[Option<wgpu::ColorTargetState>],
    vertex_entry_point: &str,
    fragment_entry_point: &str,
) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts,
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some(vertex_entry_point),
            buffers: vertex_buffer_layouts,
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some(fragment_entry_point),
            targets: fragment_targets,
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Cw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: SAMPLE_COUNT,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}
