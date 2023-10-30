mod camera;
mod hardware;
mod memory;
mod nbody;
mod pipelines;
mod settings;
mod simulation;

pub use camera::Camera;
pub use nbody::NBody;
pub use settings::Settings;

use hardware::Hardware;
use log;
use memory::{Memory, Vertex};
use pipelines::Pipelines;
use simulation::Simulation;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub async fn run(settings: Settings, camera: Camera, init_conditions: NBody) {
    debug_assert!(init_conditions.is_valid());

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Heavens")
        .with_inner_size(winit::dpi::LogicalSize::new(
            settings.display_width * settings.pixel_size,
            settings.display_height * settings.pixel_size,
        ))
        .build(&event_loop)
        .unwrap();

    let mut simulation = Simulation::new(window, settings, camera, init_conditions).await;
    let mut azimuthal_delta = 0.0;
    let mut polar_delta = 0.0;
    let mut zoom_delta = 1.0;
    let mut pause_time = false;

    event_loop.run(move |event, _, control_flow| {
        // control_flow.set_poll(); // Continuously runs the event loop
        control_flow.set_wait(); // Runs the event loop only when an event is received

        match event {
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == simulation.hardware.window.id() => {
                if !simulation.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => {
                            log::info!("Escape pressed, closing");
                            *control_flow = ControlFlow::Exit
                        }
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(key_code),
                                    ..
                                },
                            ..
                        } => match key_code {
                            VirtualKeyCode::Minus => {
                                simulation.settings.time_step /= 2.0;
                            }
                            VirtualKeyCode::Equals => {
                                simulation.settings.time_step *= 2.0;
                            }
                            VirtualKeyCode::Q => {
                                zoom_delta += 1.0e-3;
                            }
                            VirtualKeyCode::E => {
                                zoom_delta -= 1.0e-3;
                            }
                            VirtualKeyCode::Z => {
                                simulation.settings.blur_radius /= 2.0;
                            }
                            VirtualKeyCode::X => {
                                simulation.settings.blur_radius *= 2.0;
                            }
                            VirtualKeyCode::F => {
                                simulation.settings.gravitational_constant /= 2.0;
                            }
                            VirtualKeyCode::G => {
                                simulation.settings.gravitational_constant *= 2.0;
                            }
                            VirtualKeyCode::A => {
                                azimuthal_delta -= 1.0e-3;
                            }
                            VirtualKeyCode::D => {
                                azimuthal_delta += 1.0e-3;
                            }
                            VirtualKeyCode::W => {
                                polar_delta -= 1.0e-3;
                            }
                            VirtualKeyCode::S => {
                                polar_delta += 1.0e-3;
                            }
                            VirtualKeyCode::O => {
                                simulation.settings.ghost_stack_visible_limit /= 2.0;
                            }
                            VirtualKeyCode::P => {
                                simulation.settings.ghost_stack_visible_limit *= 2.0;
                            }
                            VirtualKeyCode::Space => {
                                polar_delta = 0.0;
                                azimuthal_delta = 0.0;
                                zoom_delta = 1.0;
                                pause_time = !pause_time;
                                println!("Time paused: {}", pause_time);
                            }
                            _ => {
                                println!("Unbound key pressed: {:?}", key_code);
                            }
                        },
                        WindowEvent::Resized(physical_size) => {
                            simulation.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            simulation.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::MainEventsCleared => {
                log::debug!("Main events cleared");
                // simulation.hardware.window.request_redraw();
            }
            Event::RedrawRequested(window_id) if window_id == simulation.hardware.window.id() => {
                log::debug!("Redraw requested");
                if !pause_time {
                    simulation.update();
                }

                simulation.camera.rotate_azimuthal(azimuthal_delta);
                simulation.camera.rotate_polar(polar_delta);
                simulation.camera.magnify(zoom_delta);

                match simulation.render() {
                    Ok(_) => {
                        log::debug!("Redraw complete");
                    }
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        simulation.resize(simulation.hardware.window.inner_size())
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::RedrawEventsCleared => {
                log::debug!("Redraw events cleared");
                simulation.hardware.window.request_redraw();
            }
            _ => (),
        }
    });
}
