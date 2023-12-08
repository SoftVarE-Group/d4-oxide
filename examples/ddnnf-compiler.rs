use clap::Parser;
use d4_oxide::compile_ddnnf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file to read CNF from.
    #[arg(short, long)]
    input: String,

    /// Output file to write d-DNNF to.
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    compile_ddnnf(args.input, args.output);
}
