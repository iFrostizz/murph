use std::num::ParseIntError;

// use eyre::Result;
use hex::FromHex;
use revm::OpCode as ROpCode;

struct OpCode(ROpCode);

pub type Transpiled = Vec<u8>;

pub fn parse(bytecode: String) -> Transpiled {
    let code = hex::decode(bytecode).unwrap();

    let mut push_index: u32 = 0;

    let transpiled = code
        .iter()
        .enumerate()
        .map(|(i, b)| {
            if i as u32 >= push_index {
                let op = ROpCode::try_from_u8(*b);

                let code_part: Vec<u8> = if let Some(opcode) = op {
                    let opcode = OpCode(opcode);
                    if opcode.is_push() {
                        // then next nibbles are hex
                        let size = opcode.push_size();
                        push_index = push_index + size as u32 + 1;

                        code[i..=(i + size as usize)].to_vec()
                    } else {
                        // non PUSH instructions
                        push_index = push_index + 1;
                        vec![*b]
                    }
                } else {
                    // hex nibbles
                    push_index = push_index + 1;
                    vec![*b]
                };

                code_part
            } else {
                vec![]
            }
        })
        .flatten()
        .collect::<Vec<u8>>();

    // Ok()

    transpiled
}

impl OpCode {
    fn is_push(&self) -> bool {
        let as_u8 = self.0.u8();

        return as_u8 >= 96 && as_u8 < 128;
    }

    fn push_size(&self) -> u8 {
        let as_u8 = self.0.u8();

        if as_u8 >= 96 && as_u8 < 128 {
            return as_u8 - 95;
        }

        0
    }
}
