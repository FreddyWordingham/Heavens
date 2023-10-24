use winit::{event::WindowEvent, window::Window};

use crate::{Hardware, Memory, NBody, Pipelines, Settings};

pub struct Simulation {
    pub hardware: Hardware,
    pub memory: Memory,
    pub pipelines: Pipelines,
}

impl Simulation {
    pub async fn new(window: Window, settings: Settings, initial_conditions: NBody) -> Self {
        let hardware = Hardware::new(window).await;
        let memory = Memory::new(settings, initial_conditions, &hardware.device);
        let pipelines = Pipelines::new(&hardware, &memory);

        Self {
            hardware,
            memory,
            pipelines,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.hardware.config.width = new_size.width;
            self.hardware.config.height = new_size.height;
            self.hardware
                .surface
                .configure(&self.hardware.device, &self.hardware.config);
        }
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self, settings: &Settings) {
        let mut encoder =
            self.hardware
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Compute Encoder"),
                });

        // Update settings uniform buffer
        self.hardware.queue.write_buffer(
            &self.memory.settings_uniform,
            0,
            bytemuck::cast_slice(settings.as_slice()),
        );

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Calculate Massive Forces"),
            });
            compute_pass.set_bind_group(
                0,
                &self.pipelines.calculate_massive_forces_bind_group,
                &[],
            );
            compute_pass.set_pipeline(&self.pipelines.calculate_massive_forces_pipeline);
            compute_pass.dispatch_workgroups((self.memory.num_massive_particles / 64) as u32, 1, 1);
        }
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Calculate Ghost Forces"),
            });
            compute_pass.set_bind_group(0, &self.pipelines.calculate_ghost_forces_bind_group, &[]);
            compute_pass.set_pipeline(&self.pipelines.calculate_ghost_forces_pipeline);
            compute_pass.dispatch_workgroups((self.memory.num_ghost_particles / 64) as u32, 1, 1);
        }

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Calculate Massive Velocities"),
            });
            compute_pass.set_bind_group(
                0,
                &self.pipelines.calculate_massive_velocities_bind_group,
                &[],
            );
            compute_pass.set_pipeline(&self.pipelines.calculate_massive_velocities_pipeline);
            compute_pass.dispatch_workgroups((self.memory.num_massive_particles / 64) as u32, 1, 1);
        }
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Calculate Ghost Velocities"),
            });
            compute_pass.set_bind_group(
                0,
                &self.pipelines.calculate_ghost_velocities_bind_group,
                &[],
            );
            compute_pass.set_pipeline(&self.pipelines.calculate_ghost_velocities_pipeline);
            compute_pass.dispatch_workgroups((self.memory.num_ghost_particles / 64) as u32, 1, 1);
        }

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Calculate Massive Positions"),
            });
            compute_pass.set_bind_group(
                0,
                &self.pipelines.calculate_massive_positions_bind_group,
                &[],
            );
            compute_pass.set_pipeline(&self.pipelines.calculate_massive_positions_pipeline);
            compute_pass.dispatch_workgroups((self.memory.num_massive_particles / 64) as u32, 1, 1);
        }
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Calculate Ghost Positions"),
            });
            compute_pass.set_bind_group(
                0,
                &self.pipelines.calculate_ghost_positions_bind_group,
                &[],
            );
            compute_pass.set_pipeline(&self.pipelines.calculate_ghost_positions_pipeline);
            compute_pass.dispatch_workgroups((self.memory.num_ghost_particles / 64) as u32, 1, 1);
        }

        self.hardware
            .queue
            .submit(std::iter::once(encoder.finish()));
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.hardware.surface.get_current_texture()?;

        let screen_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.hardware
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Texture"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.memory.display_view,
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
        }

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Pre-Render Ghost Particles"),
            });
            compute_pass.set_bind_group(
                0,
                &self.pipelines.pre_render_ghost_particles_bind_group,
                &[],
            );
            compute_pass.set_pipeline(&self.pipelines.pre_render_ghost_particles_pipeline);
            compute_pass.dispatch_workgroups((self.memory.num_ghost_particles / 64) as u32, 1, 1);
        }
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Render Ghost Particles"),
            });
            compute_pass.set_bind_group(0, &self.pipelines.render_ghost_particles_bind_group, &[]);
            compute_pass.set_pipeline(&self.pipelines.render_ghost_particles_pipeline);
            compute_pass.dispatch_workgroups((self.memory.num_ghost_particles / 64) as u32, 1, 1);
        }
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Render Massive Particles"),
            });
            compute_pass.set_bind_group(
                0,
                &self.pipelines.render_massive_particles_bind_group,
                &[],
            );
            compute_pass.set_pipeline(&self.pipelines.render_massive_particles_pipeline);
            compute_pass.dispatch_workgroups((self.memory.num_massive_particles / 64) as u32, 1, 1);
        }
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Vertical Gaussian Blur"),
            });
            compute_pass.set_bind_group(0, &self.pipelines.blur_vertically_bind_group, &[]);
            compute_pass.set_pipeline(&self.pipelines.blur_vertically_pipeline);
            compute_pass.dispatch_workgroups((1024 / 8) as u32, (1024 / 8) as u32, 1);
        }
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Horizontal Gaussian Blur"),
            });
            compute_pass.set_bind_group(0, &self.pipelines.blur_horizontally_bind_group, &[]);
            compute_pass.set_pipeline(&self.pipelines.blur_horizontally_pipeline);
            compute_pass.dispatch_workgroups((1024 / 8) as u32, (1024 / 8) as u32, 1);
        }
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Render Ghost Particles"),
            });
            compute_pass.set_bind_group(0, &self.pipelines.render_ghost_particles_bind_group, &[]);
            compute_pass.set_pipeline(&self.pipelines.render_ghost_particles_pipeline);
            compute_pass.dispatch_workgroups((self.memory.num_ghost_particles / 64) as u32, 1, 1);
        }
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Render Massive Particles"),
            });
            compute_pass.set_bind_group(
                0,
                &self.pipelines.render_massive_particles_bind_group,
                &[],
            );
            compute_pass.set_pipeline(&self.pipelines.render_massive_particles_pipeline);
            compute_pass.dispatch_workgroups((self.memory.num_massive_particles / 64) as u32, 1, 1);
        }

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &screen_view,
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

            render_pass.set_pipeline(&self.pipelines.display_pipeline);
            render_pass.set_bind_group(0, &self.pipelines.display_bind_group, &[]);
            render_pass.set_index_buffer(
                self.memory.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.set_vertex_buffer(0, self.memory.vertex_buffer.slice(..));
            render_pass.draw_indexed(0..self.memory.num_indices, 0, 0..1);
        }

        self.hardware
            .queue
            .submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
