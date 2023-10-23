use heavens::{run, NBody, Settings};

fn main() {
    env_logger::init();
    pollster::block_on(start());
}

async fn start() {
    println!("Initialising settings...");
    let settings = init_settings();
    println!("Generating initial conditions...");
    let init_conditions = init_conditions(settings.gravitational_constant);
    println!("Initial conditions generated.\nRunning simulation...");
    run(settings, init_conditions).await;
}

fn init_settings() -> Settings {
    Settings {
        display_width: 1024.0,
        display_height: 1024.0,
        pixel_size: 1.0,
        zoom: 0.1,
        gravitational_constant: 1.0,
        time_step: 1.0e2,
        smoothing_length: 1.0,
        ghost_mass: 1.0,
    }
}

fn init_conditions(grav_const: f32) -> NBody {
    let mut rng = rand::thread_rng();

    let mut init_conditions = NBody::new();

    // init_conditions.add_massive_disc(
    //     &mut rng,
    //     grav_const,      // gravitational constant  [m^3 kg^-1 s^-2]
    //     [0.0, 0.0, 0.0], // centre                  [m]
    //     [0.0, 0.0, 0.0], // drift                   [m/s]
    //     1.0e3,           // radius                  [m]
    //     1.0e3,           // disc mass               [kg]
    //     64 * 10,         // num particles
    // );

    init_conditions.add_ghost_field(
        &mut rng,
        [0.0, 0.0, 0.0], // centre                  [m]
        [0.0, 0.0, 0.0], // drift                   [m/s]
        1.0e3,           // radius                  [m]
        1.0e0,           // central mass           [kg]
        // 65535 * 64,      // num particles
        10000 * 64, // num particles
        1.0,        // kind (used to colour particles)
    );

    init_conditions.add_massive_system(
        &mut rng,
        [0.0, 0.0, 0.0], // centre
        [0.0, 0.0, 0.0], // drift
        1000.0,          // radius
        1.0e0,           // central mass
        1.0e-1,          // disc mass
        64 * 100,        // num particles
    );

    init_conditions.add_massive_system(
        &mut rng,
        [4000.0, -1000.0, 0.0], // centre
        [-1.0e-1, 0.0, 0.0],    // drift
        1000.0,                 // radius
        1.0e-1,                 // central mass
        1.0e-2,                 // disc mass
        64 * 10,                // num particles
    );

    init_conditions
}
