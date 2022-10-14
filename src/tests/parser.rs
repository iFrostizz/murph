#[cfg(test)]
mod parser_test {
    use std::collections::{HashMap, HashSet};

    use crate::{
        parser::{self, JumpTable},
        utils::{Byte, OpCode, ROpCode, SourceByte},
    };
    use revm::opcode;

    #[test]
    fn test_parse_add() {
        let code = String::from("61010201");
        let parsed = parser::parse(code).sb;

        assert_eq!(
            parsed,
            vec![
                SourceByte {
                    byte: vec![
                        Byte::Op(OpCode(ROpCode::try_from_u8(opcode::PUSH2))),
                        Byte::Hex(String::from("01")),
                        Byte::Hex(String::from("02"))
                    ],
                    pc: 0
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode(ROpCode::try_from_u8(opcode::ADD)))],
                    pc: 3
                },
            ]
        );
    }

    #[test]
    fn test_invalid_push() {
        let code = String::from("6100");
        let parsed = parser::parse(code).sb;

        assert_eq!(
            parsed,
            vec![SourceByte {
                byte: vec![
                    Byte::Op(OpCode(ROpCode::try_from_u8(opcode::PUSH2))),
                    Byte::Hex(String::from("00")),
                    Byte::Hex(String::from("<UNFINISHED_PUSH>"))
                ],
                pc: 0
            }]
        )
    }

    #[test]
    fn test_jump_location() {
        let code = String::from("6003565B");
        let out = parser::parse(code);
        let (parsed, jump_table) = (out.sb, out.jt);

        assert_eq!(
            parsed,
            vec![
                SourceByte {
                    byte: vec![
                        Byte::Op(OpCode(ROpCode::try_from_u8(opcode::PUSH1))),
                        Byte::Hex(String::from("03")),
                    ],
                    pc: 0
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode(ROpCode::try_from_u8(opcode::JUMP)))],
                    pc: 2
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode(ROpCode::try_from_u8(opcode::JUMPDEST)))],
                    pc: 3
                }
            ]
        );

        assert_eq!(
            jump_table,
            JumpTable {
                jump: HashMap::from([(2, 3)]),
                jumpdest: HashSet::from([3])
            }
        );
    }

    #[test]
    fn test_jumpi_location() {
        let code = String::from("632222222214601C575B");
        let out = parser::parse(code);
        let (parsed, jump_table) = (out.sb, out.jt);

        assert_eq!(
            parsed,
            vec![
                SourceByte {
                    byte: vec![
                        Byte::Op(OpCode(ROpCode::try_from_u8(opcode::PUSH4))),
                        Byte::Hex(String::from("22")),
                        Byte::Hex(String::from("22")),
                        Byte::Hex(String::from("22")),
                        Byte::Hex(String::from("22")),
                    ],
                    pc: 0
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode(ROpCode::try_from_u8(opcode::EQ)))],
                    pc: 5
                },
                SourceByte {
                    byte: vec![
                        Byte::Op(OpCode(ROpCode::try_from_u8(opcode::PUSH1))),
                        Byte::Hex(String::from("1c"))
                    ],
                    pc: 6
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode(ROpCode::try_from_u8(opcode::JUMPI)))],
                    pc: 8
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode(ROpCode::try_from_u8(opcode::JUMPDEST)))],
                    pc: 9
                },
            ]
        );

        assert_eq!(
            jump_table,
            JumpTable {
                // loc => to
                jump: HashMap::from([(8, 28)]),
                // loc
                jumpdest: HashSet::from([(9)]),
            }
        )
    }

    #[test]
    fn test_simple_store() {
        let code = String::from("60348060093d393df360003560e01c8063552410771461001c5780632096525514610023575b6004356000555b60005460005260206000f360006000fd");
        let out = parser::parse(code);
        let (parsed, jump_table) = (out.sb, out.jt);

        /*assert_eq!(
            parsed,
            vec![
                SourceByte {
                    byte: vec![
                        Byte::Op(OpCode(ROpCode::try_from_u8(opcode::PUSH1))),
                        Byte::Hex(String::from("03")),
                    ],
                    pc: 1
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode(ROpCode::try_from_u8(opcode::JUMP)))],
                    pc: 3
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode(ROpCode::try_from_u8(opcode::JUMPDEST)))],
                    pc: 4
                }
            ]
        )*/

        assert_eq!(
            jump_table,
            JumpTable {
                jump: HashMap::from([(25, 28), (36, 35)]),
                jumpdest: HashSet::from([37, 44])
            }
        )
    }
}
