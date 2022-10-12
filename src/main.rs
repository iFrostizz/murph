use clap::Parser;
use std::{fs::File, io::Write};

mod formatter;
mod parser;
mod utils;

/// Transpile bytecode into huff
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Bytecode
    #[clap(short, long)]
    bytecode: String,

    #[clap(short, long)]
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let bytecode = args.bytecode;

    let bytecode = if let Some(stripped) = bytecode.strip_prefix("0x") {
        stripped.to_string()
    } else {
        bytecode
    };

    let parsed = parser::parse(bytecode);

    let huff = formatter::to_huff(parsed);

    if let Some(path) = args.file {
        let mut file = File::create(path).unwrap();
        file.write_all(huff.as_bytes()).unwrap();
    } else {
        println!("{}", &huff);
    }
}
