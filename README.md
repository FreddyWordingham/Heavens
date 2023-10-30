# ✨ Heavens

Run N-Body simulations with your GPU:

<p align="center">
  <img src="./resources/movie.gif">
</p>

## 📦 Dependencies

-   [Rust compiler](https://www.rust-lang.org/tools/install)

## 🚀 Quickstart

Clone the repository and set your working directory to the root of the project:

```shell
git clone https://github.com/FreddyWordingham/heavens.git
cd heavens
```

Build the project in release mode:

```shell
cargo build --release
```

Run the program:

```shell
cargo run --release
```

<p align="center">
  <img src="./resources/screenshot.png">
</p>

## 📝 Usage

I will create a runtime TOML configuration scheme in the future.

For now you can use `heavens`as a library to design your own N-Body simulations:

1. You'll need these imports:

```rust
use heavens::{run, Camera, NBody, Settings};
```

2. Initialise your settings:

```rust
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
```

3. Initialise your camera:

```rust
fn init_camera() -> Camera {
    let eye_pos = [1.0e3, 0.0, 1.0e3]; // [m]
    let tar_pos = [0.0, 0.0, 0.0]; // [m]
    let field_of_view = 90.0_f32.to_radians(); // [radians]
    let zoom = 1000.0; // [m]

    Camera::new(eye_pos, tar_pos, field_of_view, zoom)
}
```

4. Now the fun part, initialise the initial conditions of your simulation:

```rust
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

    // More bodies here...

    init_conditions
}
```

5. Write the main function:

```rust
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
```

See the example [`main.rs`](./src/main.rs) for a more complete example.

## 📚 Documentation

Find the documentation at https://docs.rs/heavens/

## 🌌 TODO

-   [x] Initial conditions helper class
-   [x] Ghost particles
-   [x] Colored particles
-   [x] Camera controls
-   [ ] Write docstrings
-   [ ] Runtime parameterisation
-   [ ] No-window (capture) mode
