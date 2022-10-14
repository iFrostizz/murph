use crate::{parser::Parsed, utils::Byte};
use revm::opcode;

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
                    if let Some(in_op) = op.0 {
                        match in_op.u8() {
                            opcode::JUMP | opcode::JUMPI => (),
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
                    let op = match o.0 {
                        Some(oc) => match oc.u8() {
                            opcode::JUMP | opcode::JUMPI => {
                                let jump_type = if oc.u8() == opcode::JUMP {
                                    String::from("jump")
                                } else {
                                    String::from("jumpi")
                                };

                                if let Some(dest) = parsed.jt.jump.get(&chunk.pc) {
                                    // if current pc has a parsed dest
                                    if let Some(..) = parsed.sb.get(i - 1) {
                                        if parsed.jt.jumpdest.get(dest).is_some() {
                                            // let mut out = String::from("jump_");
                                            let mut out = jump_type;
                                            out.push_str(&format!("_{}", dest));

                                            out
                                        } else {
                                            let mut out = jump_type;
                                            out.push_str(&format!("_<{}>", dest));

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
                                    let mut out = String::from("jumpdest_");
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
                    };
                    huff.push_str(&op);
                }
            };
        }
    });

    huff.push_str("\n}");

    huff
}
