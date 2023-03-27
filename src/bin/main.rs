use clap::Parser;
use ndarray::Array2;
use std::{thread::sleep, time};

use constellation::Galaxy;

const YEAR: f64 = 365.25 * 24.0 * 60.0 * 60.0;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "1000")]
    num_stars: usize,

    #[arg(short, long, default_value = "1.0e6")]
    radius: f64,

    #[arg(short, long, default_value = "512")]
    res: usize,

    #[arg(short, long)]
    grav_strength: f64,

    #[arg(short, long)]
    smoothing_length: f64,
}

fn main() {
    println!("Hello, Galaxy!");

    let args = Args::parse();

    let mut galaxy = Galaxy::new(
        args.num_stars,
        args.radius,
        args.grav_strength,
        args.smoothing_length,
    );

    loop {
        for _ in 0..100 {
            galaxy.evolve(0.01 * YEAR);
        }

        let count = galaxy.count(args.res);
        display(&count);
        sleep(time::Duration::from_millis(10));
    }
}

/// Print the galaxy count to the terminal.
fn display(count: &Array2<u8>) {
    let mut buffer = String::new();

    for row in count.rows() {
        for &val in row.iter() {
            match val {
                0 => buffer.push_str("   "),
                1 => buffer.push_str(" . "),
                2 => buffer.push_str(" : "),
                3 => buffer.push_str(" * "),
                x => buffer.push_str(&format!(" {} ", x)),
            }
        }
        buffer.push('\n');
    }

    print!("\x1B[2J\x1B[1;1H{}", buffer);
}
