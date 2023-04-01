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
    let input = Input::build(&mut rng, &params);
    input.render(Path::new("output"), 0);
}

fn create_output_dirs(num_cameras: usize) {
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        std::fs::create_dir(output_dir).unwrap();
    }
    for i in 0..num_cameras {
        let camera_dir = output_dir.join(format!("camera_{}", i));
        if !camera_dir.exists() {
            std::fs::create_dir(camera_dir).unwrap();
        }
    }
}
