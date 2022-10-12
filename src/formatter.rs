use crate::utils::Byte;
// use revm::opcode::JUMPDEST;

pub fn to_huff(parsed: Vec<Vec<Byte>>) -> String {
    let mut huff = String::from("#define macro MAIN() = takes(0) returns(0) {");

    parsed.iter().for_each(|chunk| {
        huff.push_str("\n\u{20}\u{20}");

        if chunk.len() > 1 {
            // is push + hex
            let (_, hex) = (chunk.get(0).unwrap(), chunk.get(1..).unwrap());

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

            match chunk.get(0).unwrap() {
                Byte::Hex(h) => {
                    huff.push_str(h);
                }
                Byte::OpCode(o) => {
                    let op = o.0;
                    match op {
                        /*JUMPDEST => {
                            huff.push_str("JUMPDEST");
                        }*/
                        _ => huff.push_str(&op.as_str().to_ascii_lowercase()),
                    }
                }
            };
        }
    });

    huff.push_str("\n}");

    huff
}
