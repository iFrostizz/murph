use crate::utils::{Byte, SourceByte};
// use revm::opcode::JUMPDEST;

pub fn to_huff(parsed: Vec<SourceByte>) -> String {
    let mut huff = String::from("#define macro MAIN() = takes(0) returns(0) {");

    parsed.iter().for_each(|chunk| {
        huff.push_str("\n\u{20}\u{20}");

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

            huff.push_str(&full_hex);
        } else {
            // is either opcode or single hex

            match byte.get(0).unwrap() {
                Byte::Hex(h) => {
                    huff.push_str(h);
                }
                Byte::Op(o) => {
                    let op = match o.0 {
                        Some(oc) => oc.as_str().to_ascii_lowercase(),
                        None => String::from("invalid"),
                    };
                    huff.push_str(&op);
                }
            };
        }
    });

    huff.push_str("\n}");

    huff
}
