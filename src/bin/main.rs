use clap::Parser;

use constellation::Galaxy;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "1000")]
    num_stars: usize,

    #[arg(short, long, default_value = "1.0e6")]
    radius: f64,

    #[arg(short, long, default_value = "512")]
    res: usize,
}

fn main() {
    println!("Hello, Galaxy!");

    let args = Args::parse();

    let galaxy = Galaxy::new(args.num_stars, args.radius);
    let _image = galaxy.raster(args.res);
}
