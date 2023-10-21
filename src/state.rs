use bytemuck;
use wgpu::util::DeviceExt;
use winit::{event::WindowEvent, window::Window};

use crate::NBody;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

pub struct State {
    // Hardware and window
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,

    // Buffers
    num_massive_particles: u32,
    massive_positions_and_masses_buffer: wgpu::Buffer,

    // Render pipeline
    render_massive_positions_pipeline: wgpu::RenderPipeline,

    // Compute pipelines
    calculate_massive_positions_pipeline: wgpu::ComputePipeline,
    calculate_massive_positions_bind_group: wgpu::BindGroup,
}

impl State {
    pub async fn new(window: Window, nbody: NBody) -> Self {
        // Window size.
        let size = window.inner_size();

        // Hardware.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // Window surface.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        // GPU handle.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // Command queue.
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        // Surface configuration.
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // Massive positions and masses.
        let num_massive_particles = nbody.num_massive_particles() as u32;
        let data: Vec<[f32; 4]> = nbody
            .massive_positions()
            .iter()
            .zip(nbody.massive_masses())
            .map(|(&position, mass)| [position[0], position[1], position[2], *mass])
            .collect();
        let massive_positions_and_masses_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE,
            });
        let massive_positions_and_masses_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: 4 * std::mem::size_of::<f32>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x4,
            }],
        };

        // Compute pipelines.
        let (calculate_massive_positions_pipeline, calculate_massive_positions_bind_group) =
            create_calculate_massive_positions_pipeline_and_bind_group(
                &device,
                &massive_positions_and_masses_buffer,
            );

        // Render pipeline.
        let render_massive_positions_pipeline =
            create_render_massive_positions_pipeline_and_bind_group(
                &device,
                &config,
                massive_positions_and_masses_buffer_layout,
            );

        Self {
            num_massive_particles,
            surface,
            device,
            queue,
            config,
            size,
            window,
            massive_positions_and_masses_buffer,
            render_massive_positions_pipeline,
            calculate_massive_positions_pipeline,
            calculate_massive_positions_bind_group,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("N-Body - Calculate Massive Positions"),
            });
            compute_pass.set_bind_group(0, &self.calculate_massive_positions_bind_group, &[]);
            compute_pass.set_pipeline(&self.calculate_massive_positions_pipeline);
            compute_pass.dispatch_workgroups(self.num_massive_particles as u32, 1, 1);
        }
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_massive_positions_pipeline);
            render_pass.set_vertex_buffer(0, self.massive_positions_and_masses_buffer.slice(..));
            render_pass.draw(0..self.num_massive_particles, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn create_calculate_massive_positions_pipeline_and_bind_group(
    device: &wgpu::Device,
    massive_positions_and_masses_buffer: &wgpu::Buffer,
) -> (wgpu::ComputePipeline, wgpu::BindGroup) {
    let shader_source = include_str!("calculate_massive_positions.wgsl");

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("N-Body - Calculate Massive Positions - Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                min_binding_size: None,
                has_dynamic_offset: false,
            },
            count: None,
        }],
    });

    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("N-Body - Render Massive Positions - Shader Module"),
        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("N-Body - Render Massive Positions - Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("N-Body - Render Massive Positions - Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader_module,
        entry_point: "main",
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("N-Body - Calculate Massive Velocities - Bind Group"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: massive_positions_and_masses_buffer.as_entire_binding(),
        }],
    });

    (pipeline, bind_group)
}

fn create_render_massive_positions_pipeline_and_bind_group(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    massive_positions_and_masses_buffer_layout: wgpu::VertexBufferLayout,
) -> wgpu::RenderPipeline {
    let render_shader =
        device.create_shader_module(wgpu::include_wgsl!("render_massive_positions.wgsl"));

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &render_shader,
            entry_point: "vs_main",
            buffers: &[massive_positions_and_masses_buffer_layout],
        },
        fragment: Some(wgpu::FragmentState {
            module: &render_shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::PointList,
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

    pipeline
}
