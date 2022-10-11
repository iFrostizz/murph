use crate::utils::Byte;
use revm::opcode::JUMPDEST;

pub fn to_huff(parsed: Vec<Vec<Byte>>) -> String {
    let /*mut*/ huff = String::default();

    parsed.iter().for_each(|chunk| {
        if chunk.len() > 1 {
            // is push + hex
            let (push, hex) = (chunk.get(0).unwrap(), chunk.get(1..).unwrap());
            let hex = hex
                .iter()
                .map(|h| match h {
                    Byte::Hex(s) => s.to_string(),
                    _ => panic!("no"),
                })
                .collect::<String>();

            dbg!(&push, &hex);
        } else {
            // is either opcode or single hex

            match chunk.get(0).unwrap() {
                Byte::Hex(_h) => {}
                Byte::OpCode(o) => {
                    let op_str = o.0.u8();
                    match op_str {
                        JUMPDEST => {
                            // dbg!("JDEST");
                        }
                        _ => (),
                    }
                }
            };
        }
    });

    huff
}
