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

use nalgebra::{Matrix4, Vector3};

fn look_at(eye: Vector3<f32>, target: Vector3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
    let forward = (target - eye).normalize();
    let right = up.cross(&forward).normalize();
    let actual_up = forward.cross(&right).normalize();

    #[rustfmt::skip]
    let rotation = Matrix4::new(
        right.x, actual_up.x, forward.x, 0.0,
        right.y, actual_up.y, forward.y, 0.0,
        right.z, actual_up.z, forward.z, 0.0,
        0.0,     0.0,         0.0,       1.0
    );

    #[rustfmt::skip]
    let translation = Matrix4::new(
        1.0, 0.0, 0.0, -eye.x,
        0.0, 1.0, 0.0, -eye.y,
        0.0, 0.0, 1.0, -eye.z,
        0.0, 0.0, 0.0, 1.0
    );

    rotation * translation
}

fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Matrix4<f32> {
    let f = 1.0 / (fov_y / 2.0).tan();
    let nf = 1.0 / (near - far);

    #[rustfmt::skip]
    let mat = Matrix4::new(
        f / aspect, 0.0, 0.0,               0.0,
        0.0,        f,   0.0,               0.0,
        0.0,        0.0, (far + near) * nf, 2.0 * far * near * nf,
        0.0,        0.0, -1.0,              0.0
    );

    mat
}

fn init_settings() -> Settings {
    let eye_pos = Vector3::new(1.0e3, 1.0e3, 1.0e3);
    let tar_pos = Vector3::new(0.0, 0.0, 0.0);
    let fov_y = 45.0_f32.to_radians();
    let aspect = 16.0 / 9.0;
    let near = 0.1;
    let far = 1.0e9;

    let projection_matrix = perspective(fov_y, aspect, near, far);
    let view_matrix = look_at(eye_pos, tar_pos, Vector3::new(0.0, 0.0, 1.0));

    let mvp = projection_matrix * view_matrix;

    Settings {
        display_width: (1024.0),
        display_height: (1024.0),
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

    init_conditions.add_massive_system(
        &mut rng,
        grav_const,      // gravitational constant  [m^3 kg^-1 s^-2]
        [0.0, 0.0, 0.0], // centre                  [m]
        [0.0, 0.0, 0.0], // drift                   [m/s]
        1.0e3,           // radius                  [m]
        1.0e1,           // centre mass             [kg]
        1.0e-1,          // disc mass               [kg]
        (64 * 100) - 2,  // num particles
    );

    init_conditions.add_massive_particle(
        [3000.0, -1600.0, 0.0], // centre
        [-0.15, 0.0, 0.0],      // drift
        0.7,                    // central mass
    );

    init_conditions.add_ghost_field(
        &mut rng,
        [0.0, 0.0, 0.0], // centre                  [m]
        [0.0, 0.0, 0.0], // drift                   [m/s]
        1.0e3,           // radius                  [m]
        1.0e1,           // central mass           [kg]
        655 * 64,        // num particles
        // 1 * 64, // num particles
        1.0, // kind (used to colour particles)
    );

    // init_conditions.add_massive_system(
    //     &mut rng,
    //     [-4000.0, 3000.0, 0.0], // centre
    //     [0.01, 0.0, 0.0],       // drift
    //     1000.0,                 // radius
    //     1.0e0,                  // central mass
    //     1.0e-1,                 // disc mass
    //     (64 * 64) - 1,          // num particles
    // );

    // init_conditions.add_massive_system(
    //     &mut rng,
    //     [4000.0, -3000.0, 0.0], // centre
    //     [-0.01, 0.0, 0.0],      // drift
    //     1000.0,                 // radius
    //     1.0e0,                  // central mass
    //     1.0e-1,                 // disc mass
    //     (64 * 64) - 1,          // num particles
    // );

    init_conditions
}
