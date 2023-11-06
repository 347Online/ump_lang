use std::fmt::Display;

use crate::error::CompilerError;

use super::serialize::AsBytes;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Instr {
    Constant, // LOAD $addr; *PUSH1*
    Print,    // *POP1 (value)*; Print to stdout
    Push,     // PUSH $arg; Push value to stack
    // Pop,        // POP; Pop a value from the stack
    Assign,     //POP 2 (val, addr) insert value at address
    Exit = 255, // EXIT; Halts the program
}

impl Instr {
    pub fn arg_count(&self) -> usize {
        // Returns the number of arguments the specified instruction requires
        // Note that this is the number of distinct arguments to read, NOT the number of bytes

        match self {
            Instr::Constant => 1,
            Instr::Push => 1,
            Instr::Print => 0,
            Instr::Exit => 0,
            Instr::Assign => 2,
        }
    }
}

impl AsBytes<1> for Instr {
    type Error = CompilerError;

    fn to_bytes(self) -> [u8; 1] {
        [self as u8]
    }

    fn try_from_bytes(bytes: [u8; 1]) -> Result<Self, Self::Error> {
        let [byte] = bytes;
        let instr = match byte {
            0 => Instr::Constant,
            1 => Instr::Print,
            255 => Instr::Exit,

            x => return Err(CompilerError::InvalidInstruction(x)),
        };

        Ok(instr)
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
