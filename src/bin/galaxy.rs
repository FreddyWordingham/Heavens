use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "1000")]
    n: usize,
}

fn main() {
    println!("Hello, Galaxy!");

    let args = Args::parse();
    println!("Number of stars: {}", args.n);
}
