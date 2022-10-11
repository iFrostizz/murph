use clap::Parser;

mod parser;

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
    let transpiled = parser::parse(bytecode)/*.unwrap()*/;
    println!("{:#?}", &transpiled);
}
