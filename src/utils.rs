use crate::opcodes::OpCode;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Byte {
    Hex(String),
    Op(OpCode),
}

#[derive(Debug, Eq, PartialEq)]
pub struct SourceByte {
    pub byte: Vec<Byte>,
    pub pc: u32,
}

// TODO: those methods should not unwrap the opcode
impl OpCode {
    pub fn is_push(&self) -> bool {
        (96..128).contains(&self.0)
    }

    pub fn push_size(&self) -> u8 {
        if (96..128).contains(&self.0) {
            return self.0 - 95;
        }

        0
    }
}
