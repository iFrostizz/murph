#![feature(once_cell)]

use clap::Parser;
use opcodes::{ExpOpCode, EXP_OPCODE_JUMPMAP, OPCODE_JUMPMAP};
use std::{fs::File, io::Read, io::Write};

mod formatter;
mod opcodes;
mod parser;
mod tests;
mod utils;

/// Transpile bytecode into huff
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Bytecode
    #[clap(short, long)]
    bytecode: Option<String>,

    /// Bytecode file path
    /// Bytecode file path
    #[clap(long, conflicts_with = "bytecode")]
    bytecode_file: Option<std::path::PathBuf>,


    #[clap(short, long)]
    file: Option<String>,

    /// Strip the creation code ?
    #[clap(short, long, default_value_t = false)]
    strip: bool,

    /// Strip the creation code ?
    #[clap(short, long, default_value_t = false)]
    exp: bool,
}

fn main() {
    let args = Args::parse();

    let bytecode = if let Some(bc) = args.bytecode {
        bc
    } else if let Some(file) = args.bytecode_file {
        let mut file = File::open(file).unwrap();
        let mut bytecode = String::new();
        file.read_to_string(&mut bytecode).unwrap();
        bytecode
    } else {
        eprintln!("error: Missing bytecode argument or file path.");
        std::process::exit(1);
    };

    let exps = if args.exp {
        vec![
            ExpOpCode {
                hex: 0xb3,
                str: "tload",
            },
            ExpOpCode {
                hex: 0xb4,
                str: "tstore",
            },
        ]
    } else {
        vec![]
    };

    let mut opcode_jumpmap = OPCODE_JUMPMAP;

    exps.iter()
        .for_each(|exp| opcode_jumpmap[exp.hex as usize] = Some(exp.str));

    EXP_OPCODE_JUMPMAP.set(opcode_jumpmap).unwrap();

    let mut parsed = parser::parse(bytecode, args.strip /*, exps*/);

    let huff = formatter::to_huff(&mut parsed);

    if let Some(path) = args.file {
        let mut file = File::create(path).unwrap();
        file.write_all(huff.as_bytes()).unwrap();
    } else {
        println!("{}", &huff);
    }
}
