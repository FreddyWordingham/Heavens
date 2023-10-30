use heavens::{run, Camera, NBody, Settings};

fn main() {
    env_logger::init();
    pollster::block_on(start());
}

async fn start() {
    println!("Initialising settings...");
    let settings = init_settings();
    println!("Initialising camera...");
    let camera = init_camera();
    println!("Generating initial conditions...");
    let init_conditions = init_conditions(settings.gravitational_constant);
    println!("Initial conditions generated.\nRunning simulation...");
    run(settings, camera, init_conditions).await;
}

fn init_settings() -> Settings {
    Settings {
        display_width: 1300.0,          // [pixels]
        display_height: 1300.0,         // [pixels]
        pixel_size: 1.0,                // [screen pixel per simulation pixel]
        gravitational_constant: 1.0,    // [m^3 kg^-1 s^-2]
        time_step: 1.0e1,               // [s]
        smoothing_length: 1.0,          // [m]
        ghost_mass: 1.0,                // [kg]
        ghost_stack_visible_limit: 4.0, // This many ghosts on top of each other will have an alpha of 1.0
        blur_radius: 5.0,               // [pixels]
    }
}

fn init_camera() -> Camera {
    let eye_pos = [1.0e3, 0.0, 1.0e3]; // [m]
    let tar_pos = [0.0, 0.0, 0.0]; // [m]
    let field_of_view = 90.0_f32.to_radians(); // [radians]
    let zoom = 1000.0; // [m]

    Camera::new(eye_pos, tar_pos, field_of_view, zoom)
}

fn init_conditions(grav_const: f32) -> NBody {
    let mut rng = rand::thread_rng();

    let mut init_conditions = NBody::new(); // Construct an empty NBody simulation

    init_conditions.add_massive_system(
        &mut rng,
        grav_const,      // gravitational constant  [m^3 kg^-1 s^-2]
        [0.0, 0.0, 0.0], // centre                  [m]
        [0.0, 0.0, 0.0], // drift                   [m/s]
        1.0e3,           // radius                  [m]
        1.0e1,           // centre mass             [kg]
        1.0e-1,          // disc mass               [kg]
        (64 * 64) - 1,   // num particles
    );
    init_conditions.add_ghost_field(
        &mut rng,
        [0.0, 0.0, 0.0], // centre                  [m]
        [0.0, 0.0, 0.0], // drift                   [m/s]
        1.0e3,           // radius                  [m]
        1.0e1,           // central mass           [kg]
        655 * 64 * 4,    // num particles
        5.0,             // kind (used to colour particles)
    );

    init_conditions.add_massive_system(
        &mut rng,
        grav_const,
        [0.0, 2000.0, 400.0], // centre
        [0.07, 0.0, 0.0],     // drift
        1000.0,               // radius
        1.0e0,                // central mass
        1.0e-1,               // disc mass
        (64 * 64) - 1,        // num particles
    );
    init_conditions.add_ghost_field(
        &mut rng,
        [0.0, 2000.0, 400.0], // centre
        [0.07, 0.0, 0.0],     // drift
        1.0e3,                // radius                  [m]
        1.0e0,                // central mass           [kg]
        655 * 64,             // num particles
        3.0,                  // kind (used to colour particles)
    );

    init_conditions.add_massive_system(
        &mut rng,
        grav_const,
        [-9000.0, 1000.0, 1000.0], // centre
        [0.1, 0.0, 0.0],           // drift
        1000.0,                    // radius
        1.0e0,                     // central mass
        1.0e-1,                    // disc mass
        (64 * 64) - 1,             // num particles
    );
    init_conditions.add_ghost_field(
        &mut rng,
        [-9000.0, 1000.0, 1000.0], // centre
        [0.1, 0.0, 0.0],           // drift
        1.0e3,                     // radius                  [m]
        1.0e0,                     // central mass           [kg]
        655 * 64,                  // num particles
        1.0,                       // kind (used to colour particles)
    );

    init_conditions
}
