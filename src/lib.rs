mod nbody;
mod state;

use log;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::nbody::NBody;
use crate::state::State;

// lib.rs
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let mut nbody = NBody::new();
    nbody.add_massive_particle([0.2, 0.0, 0.0]);
    nbody.add_massive_particle([0.0, 0.0, 0.0]);
    nbody.add_massive_particle([0.0, 0.01, 0.0]);
    nbody.add_massive_particle([0.0, 0.001, 0.0]);
    nbody.add_massive_particle([-0.5, -0.5, 0.0]);
    nbody.add_massive_particle([0.5, -0.5, 0.0]);
    nbody.add_massive_particle([0.0, 0.0, 0.0]);
    nbody.add_massive_particle([0.1, 0.0, 0.0]);
    nbody.add_massive_particle([0.2, 0.0, 0.0]);
    nbody.add_massive_particle([0.3, 0.0, 0.0]);
    nbody.add_massive_particle([0.4, 0.0, 0.0]);

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        let width = 800;
        let height = 600;

        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(width, height));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    let mut state = State::new(window, nbody).await;

    event_loop.run(move |event, _, control_flow| {
        // control_flow.set_poll(); // Continuously runs the event loop
        control_flow.set_wait(); // Runs the event loop only when an event is received

        match event {
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == state.window().id() => {
                if !state.input(event) {
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
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::MainEventsCleared => {
                log::info!("Main events cleared");
                // state.window().request_redraw();
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                log::info!("Redraw requested");
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(state.size())
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::RedrawEventsCleared => {
                log::info!("Redraw events cleared");
                state.window().request_redraw();
            }
            _ => (),
        }
    });
}
