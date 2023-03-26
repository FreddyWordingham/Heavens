use clap::Parser;
use ndarray::Array2;

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
    let count = galaxy.count(args.res);

    display(&count);
}

/// Print the galaxy count to the terminal.
fn display(count: &Array2<u8>) {
    for row in count.genrows() {
        for &val in row.iter() {
            if val == 0 {
                print!("   ");
            } else {
                print!("{0:>3}", val);
            }
        }
        println!();
    }
}
