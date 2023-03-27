use clap::Parser;
use ndarray::Array2;

use heavens::{render, Galaxy};

const YEAR: f32 = 365.25 * 24.0 * 60.0 * 60.0;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "1000")]
    num_stars: usize,

    #[arg(short, long, default_value = "1.0e6")]
    radius: f32,

    #[arg(short, long, default_value = "512")]
    res: usize,

    #[arg(short, long)]
    grav_strength: f32,

    #[arg(short, long)]
    smoothing_length: f32,

    #[clap(short, long, value_parser, num_args = 2.., value_delimiter = ' ')]
    cmap: Vec<String>,
}

fn main() {
    println!("Hello, Galaxy!");

    let args = Args::parse();

    let mut rng = rand::thread_rng();
    let mut galaxy = Galaxy::new(
        &mut rng,
        args.num_stars,
        args.radius,
        args.grav_strength,
        args.smoothing_length,
        &args.cmap,
    );

    let mut step = 0;
    loop {
        let count = galaxy.count(args.res);
        let image = render::image(count, 4, &galaxy.cmap);
        let png = render::encode(&image);
        png.save(&format!("output/{:06}.png", step))
            .expect("Failed to save image.");
        // display(&count);
        // sleep(time::Duration::from_millis(10));

        step += 1;

        for _ in 0..10 {
            galaxy.evolve(&mut rng, 0.01 * YEAR);
        }
    }
}

/// Print the galaxy count to the terminal.
#[allow(dead_code)]
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
