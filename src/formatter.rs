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
                    /*dbg!(&h);
                    huff.push_str(h);*/
                    unreachable!("Having a hex without push: {}", h);
                }
                Byte::Op(o) => {
                    let op = match o.0 {
                        Some(oc) => match oc.u8() {
                            opcode::JUMP => {
                                if let Some(dest) = parsed.jt.jump.get(&chunk.pc) {
                                    if let Some(..) = parsed.sb.get(i - 1) {
                                        if parsed.jt.jumpdest.get(dest).is_some() {
                                            let mut out = String::from("jump_");
                                            out.push_str(&dest.to_string());

                                            out
                                        } else {
                                            String::from("jump_?")
                                        }
                                    } else {
                                        oc.as_str().to_ascii_lowercase()
                                    }
                                } else {
                                    panic!("jump without location");
                                    // means that no PC has been pushed before
                                }
                            }
                            opcode::JUMPDEST => {
                                if parsed.jt.jumpdest.get(&chunk.pc).is_some() {
                                    let mut out = String::from("jumpdest_");
                                    out.push_str(&chunk.pc.to_string());

                                    out
                                } else {
                                    oc.as_str().to_ascii_lowercase()
                                }
                            }
                            _ => oc.as_str().to_ascii_lowercase(),
                        },
                        None => {
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
