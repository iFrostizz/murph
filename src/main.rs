#![feature(once_cell)]

use clap::Parser;
use opcodes::{ExpOpCode, EXP_OPCODE_JUMPMAP, OPCODE_JUMPMAP};
use std::{fs::File, io::Write};

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

    let exps = vec![
        ExpOpCode {
            hex: 0xb3,
            str: "tload",
        },
        ExpOpCode {
            hex: 0xb4,
            str: "tstore",
        },
    ];

    let mut opcode_jumpmap = OPCODE_JUMPMAP;

    exps.iter()
        .for_each(|exp| opcode_jumpmap[exp.hex as usize] = Some(exp.str));

    EXP_OPCODE_JUMPMAP.set(opcode_jumpmap).unwrap();

    /*let mut final_exps = OPCODE_JUMPMAP.get_mut().unwrap();
    exps.iter()
        .for_each(|exp| final_exps[exp.hex as usize] = Some(exp.str));*/

    // let CELL: OnceCell<[Option<&'static str>; 256]> = OnceCell::with_value(final_exps);

    let mut parsed = parser::parse(bytecode, args.strip /*, exps*/);

    let huff = formatter::to_huff(&mut parsed);

    if let Some(path) = args.file {
        let mut file = File::create(path).unwrap();
        file.write_all(huff.as_bytes()).unwrap();
    } else {
        println!("{}", &huff);
    }
}
