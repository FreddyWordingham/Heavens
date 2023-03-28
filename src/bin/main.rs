use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    grav_strength: f32,

    #[arg(short, long)]
    smoothing_length: f32,

    #[clap(short, long, value_parser, num_args = 2.., value_delimiter = ' ')]
    cmap: Vec<String>,
}

fn main() {
    let args = Args::parse();
    println!("Hello, Galaxy!");
    println!("{:?}", args);
}
