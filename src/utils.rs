pub use revm::OpCode as ROpCode;

#[derive(Debug, Clone, Copy)]
pub struct OpCode(pub ROpCode);

#[derive(Debug)]
pub enum Byte {
    Hex(String),
    OpCode(OpCode),
}

impl OpCode {
    pub fn is_push(&self) -> bool {
        let as_u8 = self.0.u8();

        return as_u8 >= 96 && as_u8 < 128;
    }

    pub fn push_size(&self) -> u8 {
        let as_u8 = self.0.u8();

        if as_u8 >= 96 && as_u8 < 128 {
            return as_u8 - 95;
        }

        0
    }
}
