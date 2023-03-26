use clap::Parser;

use constellation::Galaxy;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "1000")]
    n: usize,

    #[arg(short, long, default_value = "1.0e6")]
    radius: f64,
}

fn main() {
    println!("Hello, Galaxy!");

    let args = Args::parse();

    let _galaxy = Galaxy::new(args.n, args.radius);
}
