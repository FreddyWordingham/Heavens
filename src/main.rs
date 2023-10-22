use heavens::{run, NBody, Settings};

fn main() {
    env_logger::init();
    pollster::block_on(start());
}

async fn start() {
    println!("Initialising settings...");
    let settings = Settings::default();
    println!("Generating initial conditions...");
    let init_conditions = init();
    println!("Initial conditions generated.\nRunning simulation...");
    run(settings, init_conditions).await;
}

fn init() -> NBody {
    let mut init_conditions = NBody::new();

    init_conditions.add_massive_particle([0.0, 0.0, 0.0], 1.0);
    init_conditions.add_massive_particle([0.1, 0.0, 0.0], 1.0e-3);
    init_conditions.add_massive_particle([1.0, 0.0, 0.0], 1.0e-3);
    init_conditions.add_massive_particle([2.0, 0.0, 0.0], 1.0e-3);
    init_conditions.add_massive_particle([3.0, 0.0, 0.0], 1.0e-3);
    init_conditions.add_massive_particle([4.0, 0.0, 0.0], 1.0e-3);
    init_conditions.add_massive_particle([5.0, 0.0, 0.0], 1.0e-3);
    init_conditions.add_massive_particle([0.0, 0.1, 0.0], 1.0e-3);
    init_conditions.add_massive_particle([0.0, 1.0, 0.0], 1.0e-3);
    init_conditions.add_massive_particle([0.0, 2.0, 0.0], 1.0e-3);
    init_conditions.add_massive_particle([0.0, 3.0, 0.0], 1.0e-3);
    init_conditions.add_massive_particle([0.0, 4.0, 0.0], 1.0e-3);
    init_conditions.add_massive_particle([0.0, 5.0, 0.0], 1.0e-3);

    init_conditions
}
