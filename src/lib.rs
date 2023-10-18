use log;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

fn render() {
    println!("RedrawRequested");
}

fn update(loop_number: u32, window: &mut Window) {
    println!("Update");
    if loop_number % 100 == 0 {
        println!("Redraw");
        window.request_redraw();
    }
}

fn keyboard_input(
    input: KeyboardInput,
    window: &mut Window,
    control_flow: &mut winit::event_loop::ControlFlow,
) {
    let key = input.virtual_keycode.unwrap();
    println!("Input: Keypress: {:?}", key);

    if key == VirtualKeyCode::Escape {
        println!("Escape pressed; stopping");
        window.set_title("Escape pressed");
        control_flow.set_exit();
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    let width = 800;
    let height = 600;

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new();
    let mut window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
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

    let mut loop_number = 0;
    event_loop.run(move |event, _, control_flow| {
        // control_flow.set_poll(); // Continuously runs the event loop
        control_flow.set_wait(); // Runs the event loop only when an event is received

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                control_flow.set_exit();
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                keyboard_input(input, &mut window, control_flow);
            }
            Event::MainEventsCleared => {
                update(loop_number, &mut window);
            }
            Event::RedrawRequested(_) => {
                render();
            }
            _ => (),
        }

        loop_number += 1;
    });
}
