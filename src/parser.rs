use crate::utils::{Byte, OpCode, ROpCode, SourceByte};

pub fn parse(bytecode: String) -> Vec<SourceByte> {
    let code = hex::decode(bytecode).unwrap();

    let mut push_index: u32 = 0;

    code.iter()
        .enumerate()
        .map(|(i, b)| {
            let pc = i as u32;
            if i as u32 >= push_index {
                let op = ROpCode::try_from_u8(*b);

                let code_part: SourceByte = if let Some(_opcode) = op {
                    let opcode = OpCode(op);
                    let mut ret = vec![Byte::Op(opcode)];

                    if opcode.is_push() {
                        // then next nibbles are hex
                        let size = opcode.push_size();
                        push_index = push_index + size as u32 + 1;

                        let range = (i + 1)..=(i + size as usize);

                        if range.end() > &(code.len() - 1) {
                            panic!("Unfinished PUSH at pc {}", pc);
                        }

                        ret.append(
                            &mut code[range]
                                .to_vec()
                                .iter()
                                .map(|h| Byte::Hex(format!("{:02x}", h)))
                                .collect::<Vec<Byte>>(),
                        )
                    } else {
                        // non PUSH instructions
                        push_index += 1;
                    }

                    SourceByte { byte: ret, pc }
                } else {
                    // is a wrong opcode
                    push_index += 1;
                    let opcode = ROpCode::try_from_u8(*b);
                    SourceByte {
                        byte: vec![Byte::Op(OpCode(opcode))],
                        pc,
                    }
                };

                code_part
            } else {
                SourceByte { byte: vec![], pc }
            }
        })
        .filter(|v| !v.byte.is_empty())
        .collect::<Vec<SourceByte>>()
}
