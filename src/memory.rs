use wgpu::util::DeviceExt;

use crate::{NBody, Settings};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
];

const INDICES: &[u16] = &[
    0, 1, 3, //
    1, 2, 3, //
];

pub struct Memory {
    // Counts
    pub num_massive_particles: u32,
    pub num_indices: u32,

    // Uniforms
    pub settings_uniform: wgpu::Buffer,

    // Particles
    pub massive_positions_and_masses_buffer: wgpu::Buffer,
    pub massive_velocities_and_masses_buffer: wgpu::Buffer,
    pub massive_forces_and_masses_buffer: wgpu::Buffer,

    // Textures
    pub display_texture: wgpu::Texture,
    pub display_view: wgpu::TextureView,
    pub display_sampler: wgpu::Sampler,

    // Rendering
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl<'a> Memory {
    pub fn new(settings: Settings, initial_conditions: NBody, device: &wgpu::Device) -> Self {
        debug_assert!(settings.is_valid());
        debug_assert!(initial_conditions.is_valid());

        let settings_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Settings Uniform"),
            contents: bytemuck::cast_slice(&[
                settings.display_width,
                settings.display_height,
                settings.pixel_size,
                settings.zoom,
                settings.gravitational_constant,
                settings.time_step,
                settings.smoothing_length,
            ]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Particle data
        let num_massive_particles = initial_conditions.num_massive_particles() as u32;
        let init_massive_positions_and_masses_data = initial_conditions
            .massive_positions()
            .iter()
            .zip(initial_conditions.massive_masses().iter())
            .map(|([px, py, pz], mass)| [*px, *py, *pz, *mass])
            .flatten()
            .collect::<Vec<f32>>();
        let massive_positions_and_masses_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Positions and Masses Buffer"),
                contents: bytemuck::cast_slice(&init_massive_positions_and_masses_data),
                usage: wgpu::BufferUsages::STORAGE,
            });
        let init_massive_velocities_and_masses_data = initial_conditions
            .massive_velocities()
            .iter()
            .zip(initial_conditions.massive_masses().iter())
            .map(|([vx, vy, vz], mass)| [*vx, *vy, *vz, *mass])
            .flatten()
            .collect::<Vec<f32>>();
        let massive_velocities_and_masses_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Velocities and Masses Buffer"),
                contents: bytemuck::cast_slice(&init_massive_velocities_and_masses_data),
                usage: wgpu::BufferUsages::STORAGE,
            });
        let init_massive_forces_and_masses_data = vec![0.0; (num_massive_particles * 4) as usize];
        let massive_forces_and_masses_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Forces and Masses Buffer"),
                contents: bytemuck::cast_slice(&init_massive_forces_and_masses_data),
                usage: wgpu::BufferUsages::STORAGE,
            });

        // Display texture
        let texture_size = wgpu::Extent3d {
            width: settings.display_width as u32,
            height: settings.display_height as u32,
            depth_or_array_layers: 1,
        };
        let display_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: Some("Display Texture"),
            view_formats: &[],
        });
        let display_view = display_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let display_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            // mag_filter: wgpu::FilterMode::Linear,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Rendering data
        let num_indices = INDICES.len() as u32;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            num_massive_particles,
            num_indices,
            settings_uniform,
            massive_positions_and_masses_buffer,
            massive_velocities_and_masses_buffer,
            massive_forces_and_masses_buffer,
            display_texture,
            display_view,
            display_sampler,
            vertex_buffer,
            index_buffer,
        }
    }
}
