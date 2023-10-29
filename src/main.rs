use heavens::{run, Camera, NBody, Settings};

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
    let eye_pos = [1.0e3, 0.0, 1.0e4];
    let tar_pos = [0.0, 0.0, 0.0];
    let field_of_view = 90.0_f32.to_radians();

    let camera = Camera::new(eye_pos, tar_pos, field_of_view);
    let mvp = camera.mvp();

    Settings {
        display_width: (750.0),
        display_height: (750.0),
        pixel_size: 1.0,
        camera_x: 0.0,
        camera_y: 0.0,
        zoom: 0.1,
        gravitational_constant: 1.0,
        time_step: 1.0e1,
        smoothing_length: 1.0,
        ghost_mass: 1.0,
        ghost_stack_visible_limit: 4.0,
        blur_radius: 5.0,

        mvp_xx: mvp[(0, 0)],
        mvp_xy: mvp[(0, 1)],
        mvp_xz: mvp[(0, 2)],
        mvp_xw: mvp[(0, 3)],
        mvp_yx: mvp[(1, 0)],
        mvp_yy: mvp[(1, 1)],
        mvp_yz: mvp[(1, 2)],
        mvp_yw: mvp[(1, 3)],
        mvp_zx: mvp[(2, 0)],
        mvp_zy: mvp[(2, 1)],
        mvp_zz: mvp[(2, 2)],
        mvp_zw: mvp[(2, 3)],
        mvp_wx: mvp[(3, 0)],
        mvp_wy: mvp[(3, 1)],
        mvp_wz: mvp[(3, 2)],
        mvp_ww: mvp[(3, 3)],
    }
}

fn init_conditions(grav_const: f32) -> NBody {
    let mut rng = rand::thread_rng();

    let mut init_conditions = NBody::new();

    // init_conditions.add_massive_particle(
    //     [3000.0, -1600.0, 500.0], // centre
    //     [-0.15, 0.0, 0.0],        // drift
    //     0.7,                      // central mass
    // );

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
        655 * 64,        // num particles
        5.0,             // kind (used to colour particles)
    );

    init_conditions.add_massive_system(
        &mut rng,
        grav_const,
        [0.0, 2000.0, 0.0], // centre
        [0.07, 0.0, 0.0],   // drift
        1000.0,             // radius
        1.0e0,              // central mass
        1.0e-1,             // disc mass
        (64 * 64) - 1,      // num particles
    );
    init_conditions.add_ghost_field(
        &mut rng,
        [0.0, 2000.0, 0.0], // centre
        [0.07, 0.0, 0.0],   // drift
        1.0e3,              // radius                  [m]
        1.0e0,              // central mass           [kg]
        655 * 64,           // num particles
        3.0,                // kind (used to colour particles)
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
