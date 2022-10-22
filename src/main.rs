use clap::Parser;
use std::{fs::File, io::Write};

mod formatter;
mod parser;
mod tests;
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

    /// Strip the creation code ?
    #[clap(short, long, default_value_t = false)]
    strip: bool,
}

fn main() {
    let args = Args::parse();
    let bytecode = args.bytecode;

    let bytecode = if let Some(stripped) = bytecode.strip_prefix("0x") {
        stripped.to_string()
    } else {
        bytecode
    };

    let mut parsed = parser::parse(bytecode, args.strip);

    let huff = formatter::to_huff(&mut parsed);

    if let Some(path) = args.file {
        let mut file = File::create(path).unwrap();
        file.write_all(huff.as_bytes()).unwrap();
    } else {
        println!("{}", &huff);
    }
}
