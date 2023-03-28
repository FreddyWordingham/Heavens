use clap::Parser;
use std::path::Path;

use heavens::Parameters;

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
    let _params = Parameters::load(Path::new(&args.parameters_path));
}
