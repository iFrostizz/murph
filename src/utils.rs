pub use revm::OpCode as ROpCode;

#[derive(Debug, Clone, Copy)]
pub struct OpCode(pub Option<ROpCode>);

#[derive(Debug, Clone)]
pub enum Byte {
    Hex(String),
    Op(OpCode),
}

#[derive(Debug)]
pub struct SourceByte {
    pub byte: Vec<Byte>,
    pub pc: u32,
}

impl OpCode {
    pub fn is_push(&self) -> bool {
        if let Some(op) = self.0 {
            let as_u8 = op.u8();

            (96..128).contains(&as_u8)
        } else {
            false
        }
    }

    pub fn push_size(&self) -> u8 {
        if let Some(op) = self.0 {
            let as_u8 = op.u8();

            if (96..128).contains(&as_u8) {
                return as_u8 - 95;
            }
        }

        0
    }
}
