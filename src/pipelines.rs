use crate::{Hardware, Memory, Vertex};

use wgpu::{BindGroup, ComputePipeline, Device, RenderPipeline};

pub struct Pipelines {
    // Display bind group
    pub display_bind_group: BindGroup,
    pub display_pipeline: wgpu::RenderPipeline,

    // Render massive particles
    pub render_massive_particles_pipeline: wgpu::ComputePipeline,
    pub render_massive_particles_bind_group: wgpu::BindGroup,

    // Render ghost particles
    pub pre_render_ghost_particles_pipeline: wgpu::ComputePipeline,
    pub pre_render_ghost_particles_bind_group: wgpu::BindGroup,
    pub render_ghost_particles_pipeline: wgpu::ComputePipeline,
    pub render_ghost_particles_bind_group: wgpu::BindGroup,

    // Calculate massive forces
    pub calculate_massive_forces_pipeline: wgpu::ComputePipeline,
    pub calculate_massive_forces_bind_group: wgpu::BindGroup,

    // Calculate massive velocities
    pub calculate_massive_velocities_pipeline: wgpu::ComputePipeline,
    pub calculate_massive_velocities_bind_group: wgpu::BindGroup,

    // Calculate massive positions
    pub calculate_massive_positions_pipeline: wgpu::ComputePipeline,
    pub calculate_massive_positions_bind_group: wgpu::BindGroup,

    // Calculate ghost forces
    pub calculate_ghost_forces_pipeline: wgpu::ComputePipeline,
    pub calculate_ghost_forces_bind_group: wgpu::BindGroup,

    // Calculate ghost velocities
    pub calculate_ghost_velocities_pipeline: wgpu::ComputePipeline,
    pub calculate_ghost_velocities_bind_group: wgpu::BindGroup,

    // Calculate ghost positions
    pub calculate_ghost_positions_pipeline: wgpu::ComputePipeline,
    pub calculate_ghost_positions_bind_group: wgpu::BindGroup,
}

impl Pipelines {
    pub fn new(hardware: &Hardware, memory: &Memory) -> Self {
        let (display_bind_group, display_pipeline) =
            Self::init_display_bind_group_and_pipeline(&hardware.device, &hardware.config, memory);

        let (render_massive_particles_pipeline, render_massive_particles_bind_group) =
            Self::init_render_massive_particles_pipeline_and_bind_group(hardware, memory);
        let (pre_render_ghost_particles_pipeline, pre_render_ghost_particles_bind_group) =
            Self::init_pre_render_ghost_particles_pipeline_and_bind_group(hardware, memory);
        let (render_ghost_particles_pipeline, render_ghost_particles_bind_group) =
            Self::init_render_ghost_particles_pipeline_and_bind_group(hardware, memory);

        let (calculate_massive_forces_pipeline, calculate_massive_forces_bind_group) =
            Self::init_calculate_massive_forces_pipeline_and_bind_group(hardware, memory);
        let (calculate_massive_velocities_pipeline, calculate_massive_velocities_bind_group) =
            Self::init_calculate_massive_velocities_pipeline_and_bind_group(hardware, memory);
        let (calculate_massive_positions_pipeline, calculate_massive_positions_bind_group) =
            Self::init_calculate_massive_positions_pipeline_and_bind_group(hardware, memory);

        let (calculate_ghost_forces_pipeline, calculate_ghost_forces_bind_group) =
            Self::init_calculate_ghost_forces_pipeline_and_bind_group(hardware, memory);
        let (calculate_ghost_velocities_pipeline, calculate_ghost_velocities_bind_group) =
            Self::init_calculate_ghost_velocities_pipeline_and_bind_group(hardware, memory);
        let (calculate_ghost_positions_pipeline, calculate_ghost_positions_bind_group) =
            Self::init_calculate_ghost_positions_pipeline_and_bind_group(hardware, memory);

        Self {
            display_bind_group,
            display_pipeline,
            render_massive_particles_pipeline,
            render_massive_particles_bind_group,
            pre_render_ghost_particles_pipeline,
            pre_render_ghost_particles_bind_group,
            render_ghost_particles_pipeline,
            render_ghost_particles_bind_group,
            calculate_massive_forces_pipeline,
            calculate_massive_forces_bind_group,
            calculate_massive_velocities_pipeline,
            calculate_massive_velocities_bind_group,
            calculate_massive_positions_pipeline,
            calculate_massive_positions_bind_group,
            calculate_ghost_forces_pipeline,
            calculate_ghost_forces_bind_group,
            calculate_ghost_velocities_pipeline,
            calculate_ghost_velocities_bind_group,
            calculate_ghost_positions_pipeline,
            calculate_ghost_positions_bind_group,
        }
    }

    fn init_display_bind_group_and_pipeline(
        device: &Device,
        config: &wgpu::SurfaceConfiguration,
        memory: &Memory,
    ) -> (BindGroup, RenderPipeline) {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("Display Bind Group Layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&memory.display_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&memory.display_sampler),
                },
            ],
            label: Some("Display Bind Group"),
        });

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            // TODO! Maybe move this into Memory struct?
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        };

        let shader_module =
            device.create_shader_module(wgpu::include_wgsl!("shaders/display.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Vertex Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                buffers: &[vertex_buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        (bind_group, pipeline)
    }

    fn init_render_massive_particles_pipeline_and_bind_group(
        hardware: &Hardware,
        memory: &Memory,
    ) -> (ComputePipeline, BindGroup) {
        let shader_source = include_str!("shaders/render_massive_particles.wgsl");
        let shader_module = hardware
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Render Massive Particles - Shader Module"),
                source: wgpu::ShaderSource::Wgsl(shader_source.into()),
            });

        let bind_group_layout =
            hardware
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Render Massive Particles - Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::StorageTexture {
                                access: wgpu::StorageTextureAccess::ReadWrite,
                                format: wgpu::TextureFormat::Rgba8Unorm,
                                view_dimension: wgpu::TextureViewDimension::D2,
                            },
                            count: None,
                        },
                    ],
                });

        let pipeline_layout =
            hardware
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Massive Particles - Pipeline Layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        let pipeline = hardware
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Render Massive Particles - Pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader_module,
                entry_point: "main",
            });

        let bind_group = hardware
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Render Massive Particles - Bind Group"),
                layout: &pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: memory.settings_uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: memory
                            .massive_positions_and_masses_buffer
                            .as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&memory.display_view),
                    },
                ],
            });

        (pipeline, bind_group)
    }

    fn init_pre_render_ghost_particles_pipeline_and_bind_group(
        hardware: &Hardware,
        memory: &Memory,
    ) -> (ComputePipeline, BindGroup) {
        let shader_source = include_str!("shaders/pre_render_ghost_particles.wgsl");
        let shader_module = hardware
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Pre-Render Ghost Particles - Shader Module"),
                source: wgpu::ShaderSource::Wgsl(shader_source.into()),
            });

        let bind_group_layout =
            hardware
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Pre-Render Ghost Particles - Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::StorageTexture {
                                access: wgpu::StorageTextureAccess::ReadWrite,
                                format: wgpu::TextureFormat::Rgba8Unorm,
                                view_dimension: wgpu::TextureViewDimension::D2,
                            },
                            count: None,
                        },
                    ],
                });

        let pipeline_layout =
            hardware
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Pre-Render Ghost Particles - Pipeline Layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        let pipeline = hardware
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Pre-Render Ghost Particles - Pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader_module,
                entry_point: "main",
            });

        let bind_group = hardware
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Pre-Render Ghost Particles - Bind Group"),
                layout: &pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: memory.settings_uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: memory.ghost_positions_and_kinds_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&memory.display_view),
                    },
                ],
            });

        (pipeline, bind_group)
    }

    fn init_render_ghost_particles_pipeline_and_bind_group(
        hardware: &Hardware,
        memory: &Memory,
    ) -> (ComputePipeline, BindGroup) {
        let shader_source = include_str!("shaders/render_ghost_particles.wgsl");
        let shader_module = hardware
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Render Ghost Particles - Shader Module"),
                source: wgpu::ShaderSource::Wgsl(shader_source.into()),
            });

        let bind_group_layout =
            hardware
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Render Ghost Particles - Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::StorageTexture {
                                access: wgpu::StorageTextureAccess::ReadWrite,
                                format: wgpu::TextureFormat::Rgba8Unorm,
                                view_dimension: wgpu::TextureViewDimension::D2,
                            },
                            count: None,
                        },
                    ],
                });

        let pipeline_layout =
            hardware
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Ghost Particles - Pipeline Layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        let pipeline = hardware
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Render Ghost Particles - Pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader_module,
                entry_point: "main",
            });

        let bind_group = hardware
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Render Ghost Particles - Bind Group"),
                layout: &pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: memory.settings_uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: memory.ghost_positions_and_kinds_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&memory.display_view),
                    },
                ],
            });

        (pipeline, bind_group)
    }

    fn init_calculate_massive_forces_pipeline_and_bind_group(
        hardware: &Hardware,
        memory: &Memory,
    ) -> (ComputePipeline, BindGroup) {
        let shader_source = include_str!("shaders/calculate_massive_forces.wgsl");
        let shader_module = hardware
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Calculate Massive Forces - Shader Module"),
                source: wgpu::ShaderSource::Wgsl(shader_source.into()),
            });

        let bind_group_layout =
            hardware
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Calculate Massive Forces - Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                    ],
                });

        let pipeline_layout =
            hardware
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Calculate Massive Forces - Pipeline Layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        let pipeline = hardware
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Calculate Massive Forces - Pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader_module,
                entry_point: "main",
            });

        let bind_group = hardware
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Calculate Massive Forces - Bind Group"),
                layout: &pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: memory.settings_uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: memory
                            .massive_positions_and_masses_buffer
                            .as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: memory.massive_forces_and_masses_buffer.as_entire_binding(),
                    },
                ],
            });

        (pipeline, bind_group)
    }

    fn init_calculate_massive_velocities_pipeline_and_bind_group(
        hardware: &Hardware,
        memory: &Memory,
    ) -> (ComputePipeline, BindGroup) {
        let shader_source = include_str!("shaders/calculate_massive_velocities.wgsl");
        let shader_module = hardware
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Calculate Massive Velocities - Shader Module"),
                source: wgpu::ShaderSource::Wgsl(shader_source.into()),
            });

        let bind_group_layout =
            hardware
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Calculate Massive Velocities - Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                    ],
                });

        let pipeline_layout =
            hardware
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Calculate Massive Velocities - Pipeline Layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        let pipeline = hardware
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Calculate Massive Velocities - Pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader_module,
                entry_point: "main",
            });

        let bind_group = hardware
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Calculate Massive Velocities - Bind Group"),
                layout: &pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: memory.settings_uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: memory.massive_forces_and_masses_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: memory
                            .massive_velocities_and_masses_buffer
                            .as_entire_binding(),
                    },
                ],
            });

        (pipeline, bind_group)
    }

    fn init_calculate_massive_positions_pipeline_and_bind_group(
        hardware: &Hardware,
        memory: &Memory,
    ) -> (ComputePipeline, BindGroup) {
        let shader_source = include_str!("shaders/calculate_massive_positions.wgsl");
        let shader_module = hardware
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Calculate Massive Positions - Shader Module"),
                source: wgpu::ShaderSource::Wgsl(shader_source.into()),
            });

        let bind_group_layout =
            hardware
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Calculate Massive Positions - Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                    ],
                });

        let pipeline_layout =
            hardware
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Calculate Massive Positions - Pipeline Layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        let pipeline = hardware
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Calculate Massive Positions - Pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader_module,
                entry_point: "main",
            });

        let bind_group = hardware
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Calculate Massive Positions - Bind Group"),
                layout: &pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: memory.settings_uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: memory
                            .massive_velocities_and_masses_buffer
                            .as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: memory
                            .massive_positions_and_masses_buffer
                            .as_entire_binding(),
                    },
                ],
            });

        (pipeline, bind_group)
    }

    fn init_calculate_ghost_forces_pipeline_and_bind_group(
        hardware: &Hardware,
        memory: &Memory,
    ) -> (ComputePipeline, BindGroup) {
        let shader_source = include_str!("shaders/calculate_ghost_forces.wgsl");
        let shader_module = hardware
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Calculate Ghost Forces - Shader Module"),
                source: wgpu::ShaderSource::Wgsl(shader_source.into()),
            });

        let bind_group_layout =
            hardware
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Calculate Ghost Forces - Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 3,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                    ],
                });

        let pipeline_layout =
            hardware
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Calculate Ghost Forces - Pipeline Layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        let pipeline = hardware
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Calculate Ghost Forces - Pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader_module,
                entry_point: "main",
            });

        let bind_group = hardware
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Calculate Ghost Forces - Bind Group"),
                layout: &pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: memory.settings_uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: memory.ghost_positions_and_kinds_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: memory
                            .massive_positions_and_masses_buffer
                            .as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: memory.ghost_forces_and_kinds_buffer.as_entire_binding(),
                    },
                ],
            });

        (pipeline, bind_group)
    }

    fn init_calculate_ghost_velocities_pipeline_and_bind_group(
        hardware: &Hardware,
        memory: &Memory,
    ) -> (ComputePipeline, BindGroup) {
        let shader_source = include_str!("shaders/calculate_ghost_velocities.wgsl");
        let shader_module = hardware
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Calculate Ghost Velocities - Shader Module"),
                source: wgpu::ShaderSource::Wgsl(shader_source.into()),
            });

        let bind_group_layout =
            hardware
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Calculate Ghost Velocities - Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                    ],
                });

        let pipeline_layout =
            hardware
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Calculate Ghost Velocities - Pipeline Layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        let pipeline = hardware
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Calculate Ghost Velocities - Pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader_module,
                entry_point: "main",
            });

        let bind_group = hardware
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Calculate Ghost Velocities - Bind Group"),
                layout: &pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: memory.settings_uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: memory.ghost_forces_and_kinds_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: memory.ghost_velocities_and_kinds_buffer.as_entire_binding(),
                    },
                ],
            });

        (pipeline, bind_group)
    }

    fn init_calculate_ghost_positions_pipeline_and_bind_group(
        hardware: &Hardware,
        memory: &Memory,
    ) -> (ComputePipeline, BindGroup) {
        let shader_source = include_str!("shaders/calculate_ghost_positions.wgsl");
        let shader_module = hardware
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Calculate Ghost Positions - Shader Module"),
                source: wgpu::ShaderSource::Wgsl(shader_source.into()),
            });

        let bind_group_layout =
            hardware
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Calculate Ghost Positions - Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                    ],
                });

        let pipeline_layout =
            hardware
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Calculate Ghost Positions - Pipeline Layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        let pipeline = hardware
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Calculate Ghost Positions - Pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader_module,
                entry_point: "main",
            });

        let bind_group = hardware
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Calculate Ghost Positions - Bind Group"),
                layout: &pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: memory.settings_uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: memory.ghost_velocities_and_kinds_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: memory.ghost_positions_and_kinds_buffer.as_entire_binding(),
                    },
                ],
            });

        (pipeline, bind_group)
    }
}
