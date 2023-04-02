use clap::Parser;
use std::path::Path;

use heavens::{Input, Parameters};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    parameters_path: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello, Galaxy!");
    println!("{:?}", args);
    let params = Parameters::load(Path::new(&args.parameters_path));
    create_output_dirs(params.cameras.len());
    let mut rng = rand::thread_rng();
    let mut input = Input::build(&mut rng, &params);
    for n in 0..params.num_steps {
        println!("Step: {}", n);
        input.render(Path::new("output"), n);
        input.evolve(params.dt);
    }
    input.render(Path::new("output"), params.num_steps);
}

fn create_output_dirs(num_cameras: usize) {
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        std::fs::create_dir(output_dir).unwrap();
    }
    for i in 0..num_cameras {
        let camera_dir = output_dir.join(format!("camera_{:03}", i));
        if !camera_dir.exists() {
            std::fs::create_dir(camera_dir).unwrap();
        }
    }
}
