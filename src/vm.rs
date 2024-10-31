use crate::instruction::*;
use crate::errors::*;
type Value = f64;

pub struct VM {
    stack: Vec<Value>,
    program: Vec<u8>,
    consts: Vec<Value>,
    pc: usize,
    running: bool,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: vec![],
            program: vec![],
            consts: vec![],
            pc: 0,
            running: false,
        }
    }

    pub fn add_const(&mut self, value: Value) {
        self.consts.push(value);
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        self.program = program;
    }

    pub fn run(&mut self) -> Result<(), InterpretationError> {
        self.running = true;
        while self.running {
            self.run_one_instr()?;
        }
        Ok(())
    }

    fn run_one_instr(&mut self) -> Result<(), InterpretationError> {
        let inst: Instruction = self.program.get(self.pc)
                                            .ok_or_else(|| InterpretationError::UnexpectedEndError(UnexpectedEndError))?
                                            .clone()
                                            .try_into()?;
        match inst {
            Instruction::NOP => { },
            Instruction::PUSH => {
                let b1: u8 = *self.program.get(self.pc+1)
                                          .ok_or_else(|| InterpretationError::UnexpectedEndError(UnexpectedEndError))?;
                let b2: u8 = *self.program.get(self.pc+2)
                                          .ok_or_else(|| InterpretationError::UnexpectedEndError(UnexpectedEndError))?;
                let index: u16 = u16::from_ne_bytes([b1, b2]);
                let val: Value = *self.consts.get(index as usize)
                                             .ok_or_else(|| InterpretationError::BadConstsIndexError(BadConstsIndexError))?;
                self.stack.push(val);
                self.pc += 3;
                
            },
            Instruction::ADD => {
                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                self.stack.push(a + b);
                self.pc += 1;
            },
            Instruction::SUB => {
                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                self.stack.push(b - a);
                self.pc += 1;
            },
            Instruction::MUL => {
                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                self.stack.push(a * b);
                self.pc += 1;
            },
            Instruction::DIV => {
                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                let b = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                if a == 0f64 {
                    return Err(InterpretationError::ZeroDivisionError(ZeroDivisionError));
                }
                self.stack.push(b / a);
                self.pc += 1;
            },
            Instruction::JMP => todo!("Не реализованы"),
            Instruction::JE => todo!("Не реализованы"),
            Instruction::JNE => todo!("Не реализованы"),
            Instruction::JG => todo!("Не реализованы"),
            Instruction::JL => todo!("Не реализованы"),
            Instruction::JGE => todo!("Не реализованы"),
            Instruction::JLE => todo!("Не реализованы"),
            Instruction::RET => todo!("Не реализованы"),
            Instruction::DBG => {
                let a = self.stack.pop()
                                  .ok_or_else(|| InterpretationError::EmptyStackError(EmptyStackError))?;
                println!("{a:#}");
                self.pc += 1;
            },
            Instruction::HLT => {
                self.running = false;
            },
        }
        Ok(())
    }
}