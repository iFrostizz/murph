use clap::Parser;

mod formatter;
mod parser;
mod utils;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    bytecode: String,
}

fn main() {
    let args = Args::parse();
    let bytecode = args.bytecode;

    let parsed = parser::parse(bytecode);

    let huff = formatter::to_huff(parsed);

    println!("{:#?}", &huff);
}
