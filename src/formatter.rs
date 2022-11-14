use crate::{
    opcodes::{JUMP, JUMPDEST, JUMPI},
    parser::Parsed,
    utils::Byte,
};

pub fn to_huff(parsed: &mut Parsed) -> String {
    let mut huff = String::from("#define macro MAIN() = takes(0) returns(0) {");

    parsed.sb.iter().enumerate().for_each(|(i, chunk)| {
        let byte = &chunk.byte;

        if byte.len() > 1 {
            // is push + hex
            let (_, hex) = (byte.get(0).unwrap(), byte.get(1..).unwrap());

            let hex = hex
                .iter()
                .map(|h| match h {
                    Byte::Hex(s) => s.to_string(),
                    _ => panic!("wtf?!"),
                })
                .collect::<String>();

            let mut full_hex = String::from("0x");
            full_hex.push_str(&hex);

            // If next is a push, don't push to huff
            if let Some(c) = parsed.sb.get(i + 1) {
                if let Byte::Op(op) = c.byte.get(0).unwrap() {
                    match op.0 {
                        JUMP | JUMPI => (),
                        _ => {
                            huff.push_str("\n\u{20}\u{20}");
                            huff.push_str(&full_hex);
                        }
                    }
                } else {
                    huff.push_str("\n\u{20}\u{20}");
                    huff.push_str(&full_hex);
                }
            } else {
                huff.push_str("\n\u{20}\u{20}");
                huff.push_str(&full_hex);
            }
        } else {
            // is either opcode or single hex

            huff.push_str("\n\u{20}\u{20}");
            match byte.get(0).unwrap() {
                Byte::Hex(h) => {
                    // huff.push_str(h);
                    unreachable!("Having a hex without push: {}", h);
                }
                Byte::Op(o) => {
                    // Can be valid or invalid but has to be an opcode
                    let op = if o.is_valid() {
                        match o.0 {
                            JUMP | JUMPI => {
                                // let jump_type = get_jump_type(oc.u8()).unwrap();
                                // TODO: should be label then jump / jumpi
                                let jump_type = String::from("jump");

                                if let Some(dest) = parsed.jt.jump.get(&chunk.pc) {
                                    // if current pc has a parsed dest
                                    if let Some(..) = parsed.sb.get(i - 1) {
                                        if parsed.jt.jumpdest.get(&dest.pc).is_some() {
                                            let mut out = jump_type;
                                            out.push_str(&format!("_{}\n\u{20}\u{20}", dest.pc));
                                            out.push_str(&o.as_str().to_ascii_lowercase());

                                            out
                                        } else {
                                            let mut out = jump_type;
                                            out.push_str(&format!("_<{}>\n\u{20}\u{20}", dest.pc));
                                            out.push_str(&o.as_str().to_ascii_lowercase());

                                            out
                                        }
                                    } else {
                                        unreachable!("Jump without dest at pc {}", &chunk.pc);
                                    }
                                } else {
                                    let mut out = jump_type;
                                    out.push_str(&format!("_<!{}>", chunk.pc));

                                    out
                                }
                            }
                            JUMPDEST => {
                                if parsed.jt.jumpdest.get(&chunk.pc).is_some() {
                                    let mut out = String::from("jump_");

                                    out.push_str(&format!("{}:", chunk.pc));

                                    out
                                } else {
                                    o.as_str().to_ascii_lowercase()
                                }
                            }
                            _ => o.as_str().to_ascii_lowercase(),
                        }
                    } else {
                        let mut inv = String::from("invalid");
                        inv.push_str(&format!("_{:x}", o.0));
                        inv
                    };
                    /*let op = match o.0 {
                        Some(oc) => match oc.u8() {
                            opcode::JUMP | opcode::JUMPI => {
                                // let jump_type = get_jump_type(oc.u8()).unwrap();
                                // TODO: should be label then jump / jumpi
                                let jump_type = String::from("jump");

                                if let Some(dest) = parsed.jt.jump.get(&chunk.pc) {
                                    // if current pc has a parsed dest
                                    if let Some(..) = parsed.sb.get(i - 1) {
                                        if parsed.jt.jumpdest.get(&dest.pc).is_some() {
                                            // let mut out = String::from("jump_");
                                            let mut out = jump_type;
                                            out.push_str(&format!("_{}\n\u{20}\u{20}", dest.pc));
                                            out.push_str(&oc.as_str().to_ascii_lowercase());

                                            out
                                        } else {
                                            let mut out = jump_type;
                                            out.push_str(&format!("_<{}>\n\u{20}\u{20}", dest.pc));
                                            out.push_str(&oc.as_str().to_ascii_lowercase());

                                            out
                                        }
                                    } else {
                                        // oc.as_str().to_ascii_lowercase()
                                        unreachable!("Jump without dest at pc {}", &chunk.pc);
                                    }
                                } else {
                                    // panic!("{} without location at pc {}", jump_type, chunk.pc);
                                    // means that no PC has been pushed before

                                    let mut out = jump_type;
                                    out.push_str(&format!("_<!{}>", chunk.pc));

                                    out
                                }
                            }
                            opcode::JUMPDEST => {
                                if parsed.jt.jumpdest.get(&chunk.pc).is_some() {
                                    // find all jumps going to this location
                                    /* let jump_locs = parsed
                                    .jt
                                    .jump
                                    .values()
                                    .map(|dest| dest.pc)
                                    .filter(|pc| *pc == chunk.pc)
                                    .collect::<Vec<u32>>();*/
                                    // let mut out = get_jump_type(10).unwrap();
                                    // let mut out = String::from("jumpdest_");

                                    let mut out = String::from("jump_");

                                    out.push_str(&format!("{}:", chunk.pc));

                                    out
                                } else {
                                    oc.as_str().to_ascii_lowercase()
                                }
                            }
                            _ => oc.as_str().to_ascii_lowercase(),
                        },
                        None => {
                            // TODO: no None opcode and let the u8 val
                            /*let inv = String::from("invalid");
                            inv.push_str(&format!("<{}>", o.0.u8()));
                            inv*/

                            String::from("invalid")
                        }
                    };*/
                    huff.push_str(&op);
                }
            };
        }
    });

    huff.push_str("\n}");

    huff
}

/*fn get_jump_type(op: u8) -> Option<String> {
    match op {
        opcode::JUMP => Some(String::from("jump")),
        opcode::JUMPI => Some(String::from("jumpi")),
        _ => None,
    }
}*/
