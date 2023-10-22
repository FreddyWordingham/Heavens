use heavens::{run, NBody, Settings};

fn main() {
    env_logger::init();
    pollster::block_on(start());
}

async fn start() {
    println!("Initialising settings...");
    let settings = init_settings();
    println!("Generating initial conditions...");
    let init_conditions = init_conditions();
    println!("Initial conditions generated.\nRunning simulation...");
    run(settings, init_conditions).await;
}

fn init_settings() -> Settings {
    Settings {
        display_width: 1024.0,
        display_height: 1024.0,
        pixel_size: 1.0,
        zoom: 1.0e1,

        gravitational_constant: 1.0,
        time_step: 1.0e-1,
        smoothing_length: 1.0e-2,
    }
}

fn init_conditions() -> NBody {
    let mut rng = rand::thread_rng();

    let mut init_conditions = NBody::new();

    init_conditions.add_massive_system(
        &mut rng,
        [50.0, 10.0, 0.0],   // centre
        [-1.0e-1, 0.0, 0.0], // drift
        10.0,                // radius
        1.0e0,               // central mass
        1.0e-1,              // disc mass
        64 * 100,            // num particles
    );

    init_conditions.add_massive_system(
        &mut rng,
        [-50.0, -10.0, 0.0], // centre
        [1.0e-1, 0.0, 0.0],  // drift
        10.0,                // radius
        1.0e0,               // central mass
        1.0e-1,              // disc mass
        64 * 100,            // num particles
    );

    init_conditions
}
