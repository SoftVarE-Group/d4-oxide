use clap::Parser;
use d4_oxide::{compile_ddnnf, compile_ddnnf_proj};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file to read CNF from.
    #[arg(short, long)]
    input: String,

    /// Output file to write d-DNNF to.
    #[arg(short, long)]
    output: String,

    /// Whether to use projected compilation.
    #[arg(short, long)]
    projected: bool,
}

fn main() {
    let args = Args::parse();
    if args.projected {
        compile_ddnnf_proj(args.input, args.output);
    } else {
        compile_ddnnf(args.input, args.output);
    }
}
