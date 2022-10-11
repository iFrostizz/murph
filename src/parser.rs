use crate::utils::{Byte, OpCode, ROpCode};

pub fn parse(bytecode: String) -> Vec<Vec<Byte>> {
    let code = hex::decode(bytecode).unwrap();

    let mut push_index: u32 = 0;

    code.iter()
        .enumerate()
        .map(|(i, b)| {
            if i as u32 >= push_index {
                let op = ROpCode::try_from_u8(*b);

                let code_part: Vec<Byte> = if let Some(opcode) = op {
                    let opcode = OpCode(opcode);
                    let mut ret = vec![Byte::OpCode(opcode)];

                    if opcode.is_push() {
                        // then next nibbles are hex
                        let size = opcode.push_size();
                        push_index = push_index + size as u32 + 1;

                        ret.append(
                            &mut code[(i + 1)..=(i + size as usize)]
                                .to_vec()
                                .iter()
                                .map(|h| Byte::Hex(format!("{:x}", h)))
                                .collect::<Vec<Byte>>(),
                        )
                    } else {
                        // non PUSH instructions
                        push_index = push_index + 1;
                    }

                    ret
                } else {
                    // hex nibbles
                    push_index = push_index + 1;
                    vec![Byte::Hex(format!("{:x}", *b))]
                };

                code_part
            } else {
                vec![]
            }
        })
        .filter(|v| !v.is_empty())
        .collect::<Vec<Vec<Byte>>>()
}
