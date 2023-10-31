use umpteen::{repr::{chunk::Chunk, instr::Instruction, value::Value, Result}, vm::Vm};

fn main() -> Result<Value> {
    // let mut stack = vec![];
    let mut chunk = Chunk::new();
    let addr = chunk.write_val(Value::Number(10.7));
    chunk.write_instr(Instruction::Constant);
    chunk.write_byte(addr as u8);
    chunk.write_instr(Instruction::Print);
    // chunk.exec(&mut stack)

    let mut vm = Vm::new();
    vm.write_chunk(chunk);
    vm.exec()
    
}

#[cfg(test)]
mod tests {
    use umpteen::Umpteen;

    #[test]
    fn test_hello_world() {
        let _ = Umpteen::run("print 10;").unwrap();
    }

    #[test]
    fn test_let_x_equal_10() {
        let _ = Umpteen::run("let x = 10;").unwrap();
    }
}
