use revm::opcode::{JUMP, JUMPDEST, JUMPI};
use std::collections::{HashMap, HashSet};

use crate::utils::{Byte, OpCode, ROpCode, SourceByte};

#[derive(Default, Debug)]
pub struct JumpTable {
    pub jumpdest: HashSet<u32>,
    /// pc => dest
    pub jump: HashMap<u32, u32>,
}

pub struct Parsed {
    pub sb: Vec<SourceByte>,
    pub jt: JumpTable,
}

pub fn parse(bytecode: String) -> Parsed {
    let code = hex::decode(bytecode).unwrap();

    let mut parsed: Vec<SourceByte> = Vec::new();

    let mut push_index: u32 = 0;
    let mut pc: u32 = 1;

    let mut jump_table = JumpTable::default();

    code.iter().enumerate().for_each(|(i, b)| {
        if i as u32 >= push_index {
            let op = ROpCode::try_from_u8(*b);

            let code_part: SourceByte = if let Some(_opcode) = op {
                let opcode = OpCode(op);
                let mut ret = vec![Byte::Op(opcode)];

                if opcode.is_push() {
                    // then next nibbles are hex
                    let size = opcode.push_size();
                    push_index = push_index + size as u32 + 1;

                    let mut range = (i + 1)..=(i + size as usize);
                    let mut unfinished = false;

                    if range.end() > &(code.len() - 1) {
                        // panic!("Unfinished PUSH at pc {}", pc);
                        range = (i + 1)..=(code.len() - 1);
                        unfinished = true;
                    }

                    ret.append(
                        &mut code[range]
                            .to_vec()
                            .iter()
                            .map(|h| Byte::Hex(format!("{:02x}", h)))
                            .collect::<Vec<Byte>>(),
                    );

                    if unfinished {
                        ret.append(&mut vec![Byte::Hex(String::from("<UNFINISHED_PUSH>"))]);
                    }
                } else {
                    // non PUSH instructions
                    let op_val = opcode.0.unwrap().u8();

                    if op_val == JUMPDEST {
                        jump_table.jumpdest.insert(pc);
                    } else if op_val == JUMP {
                        if let Some(source_hex) = parsed.last() {
                            if let Some(Byte::Op(op)) = source_hex.byte.get(0) {
                                if op.is_push() {
                                    let push_size = op.push_size();
                                    let hex = source_hex
                                        .byte
                                        .get(1..(push_size + 1) as usize)
                                        .unwrap()
                                        .iter()
                                        .map(|b| {
                                            if let Byte::Hex(h) = b {
                                                h.to_string()
                                            } else {
                                                panic!("PUSH should prepend hex");
                                            }
                                        })
                                        .collect::<String>();
                                    let dest = u32::from_str_radix(&hex, 16).unwrap();
                                    jump_table.jump.insert(pc, dest);
                                }
                            }
                        }
                    } else if op_val == JUMPI {
                        /*if let Some(source_hex) = parsed.last() {
                            if let Some(Byte::Op(op)) = source_hex.byte.get(0) {
                                if op.is_push() {
                                    let push_size = op.push_size();
                                    let hex = source_hex
                                        .byte
                                        .get(1..(push_size + 1) as usize)
                                        .unwrap()
                                        .iter()
                                        .map(|b| {
                                            if let Byte::Hex(h) = b {
                                                h.to_string()
                                            } else {
                                                panic!("PUSH should prepend hex");
                                            }
                                        })
                                        .collect::<String>();
                                    let pc = u32::from_str_radix(&hex, 16).unwrap();
                                    jump_table.jump.insert(pc);
                                }
                            }
                        }*/
                    }

                    push_index += 1;
                }

                SourceByte { byte: ret, pc }
            } else {
                // is ending a PUSH
                push_index += 1;
                let opcode = ROpCode::try_from_u8(*b);

                SourceByte {
                    byte: vec![Byte::Op(OpCode(opcode))],
                    pc,
                }
            };

            pc += 1;
            parsed.push(code_part);
        }
    });

    Parsed {
        sb: parsed,
        jt: jump_table,
    }
}
