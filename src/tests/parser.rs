#[cfg(test)]
mod parser_test {
    use crate::{
        parser,
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
                    pc: 1
                },
                SourceByte {
                    byte: vec![Byte::Op(OpCode(ROpCode::try_from_u8(opcode::ADD)))],
                    pc: 2
                },
            ]
        );
    }
}
