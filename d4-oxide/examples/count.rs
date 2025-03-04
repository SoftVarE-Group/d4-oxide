use clap::Parser;
use d4_oxide::{count, count_proj};

#[derive(Parser, Debug)]
struct Args {
    /// Input file to read CNF from.
    #[arg(short, long)]
    input: String,

    /// Whether to use projected compilation.
    #[arg(short, long)]
    projected: bool,
}

fn main() {
    let args = Args::parse();
    if args.projected {
        println!("{}", count_proj(args.input));
    } else {
        println!("{}", count(args.input));
    }
}
