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
        (0x5f..0x80).contains(&self.0)
    }

    pub fn push_size(&self) -> u8 {
        if !self.is_push() {
            return 0;
        }
        
        // For PUSH0 (0x5f), size is 0
        // For PUSH1-PUSH32 (0x60-0x7f), size is (opcode - 0x5f)
        self.0.saturating_sub(0x5f)
    }
}
