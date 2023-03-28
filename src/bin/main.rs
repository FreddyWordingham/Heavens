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
    let mut rng = rand::thread_rng();
    let _input = Input::build(&mut rng, &params);
}
