use std::io;
use thiserror::Error;

pub struct Machine {
    pub memory: [isize; 2000],
    pub pointer_addr: usize,
    pub relative_base: isize,
}

#[derive(Debug)]
enum Opcode {
    ADD(ParamMode, ParamMode, ParamMode),
    MUL(ParamMode, ParamMode, ParamMode),
    STR(ParamMode),
    LDR(ParamMode),
    JMPT(ParamMode, ParamMode),
    JMPF(ParamMode, ParamMode),
    LT(ParamMode, ParamMode, ParamMode),
    EQ(ParamMode, ParamMode, ParamMode),
    ADJ(ParamMode),
    END,
}

#[derive(Debug, Copy, Clone)]
enum ParamMode {
    M0,
    M1,
    M2,
}

#[derive(Error, Debug)]
pub enum MachineError {
    #[error("Address {0} is invalid")]
    InvalidAddress(isize),
    #[error("Opcode {0} is invalid")]
    InvalidOpcodeError(String),
    #[error("Out of memory")]
    OutOfMemoryError,
}

impl Machine {
    pub fn load_initial_memory(&mut self, initial_memory: &[isize]) -> Result<(), MachineError> {
        if self.memory.len() < initial_memory.len() {
            Err(MachineError::OutOfMemoryError)
        } else {
            self.memory[..initial_memory.len()].copy_from_slice(initial_memory);
            Ok(())
        }
    }

    pub fn run(&mut self) -> Result<(), MachineError> {
        while let Ok(ins) = parse_opcode(self.memory[self.pointer_addr]) {
            match ins {
                Opcode::ADD(param_mode_1, param_mode_2, param_mode_3) => {
                    self.add(param_mode_1, param_mode_2, param_mode_3)?;
                }
                Opcode::MUL(param_mode_1, param_mode_2, param_mode_3) => {
                    self.mul(param_mode_1, param_mode_2, param_mode_3)?;
                }
                Opcode::STR(param_mode) => {
                    let mut input = String::new();
                    println!("Please type a number: ");
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let input: isize = input.trim().parse().expect("Please type a number: ");
                    self.str(param_mode, input)?;
                }
                Opcode::LDR(param_mode) => {
                    println!("Output value: {}", self.ldr(param_mode)?);
                }
                Opcode::JMPT(param_mode_1, param_mode_2) => {
                    self.jmpt(param_mode_1, param_mode_2)?;
                }
                Opcode::JMPF(param_mode_1, param_mode_2) => {
                    self.jmpf(param_mode_1, param_mode_2)?;
                }
                Opcode::LT(param_mode_1, param_mode_2, param_mode_3) => {
                    self.lt(param_mode_1, param_mode_2, param_mode_3)?;
                }
                Opcode::EQ(param_mode_1, param_mode_2, param_mode_3) => {
                    self.eq(param_mode_1, param_mode_2, param_mode_3)?;
                }
                Opcode::ADJ(param_mode) => {
                    self.adj(param_mode)?;
                }
                Opcode::END => break,
            }
        }
        Ok(())
    }

    fn add(
        &mut self,
        param_mode_1: ParamMode,
        param_mode_2: ParamMode,
        param_mode_3: ParamMode,
    ) -> Result<(), MachineError> {
        let val_1 = self.get_val(param_mode_1, 1)?;
        let val_2 = self.get_val(param_mode_2, 2)?;
        let addr = self.get_addr(param_mode_3, 3)?;
        self.memory[addr] = val_1 + val_2;
        self.pointer_addr += 4;
        Ok(())
    }

    fn mul(
        &mut self,
        param_mode_1: ParamMode,
        param_mode_2: ParamMode,
        param_mode_3: ParamMode,
    ) -> Result<(), MachineError> {
        let val_1 = self.get_val(param_mode_1, 1)?;
        let val_2 = self.get_val(param_mode_2, 2)?;
        let addr = self.get_addr(param_mode_3, 3)?;
        self.memory[addr] = val_1 * val_2;
        self.pointer_addr += 4;
        Ok(())
    }

    fn str(&mut self, param_mode: ParamMode, input: isize) -> Result<(), MachineError> {
        let addr = self.get_addr(param_mode, 1)?;
        self.memory[addr] = input;
        self.pointer_addr += 2;
        Ok(())
    }

    fn ldr(&mut self, param_mode: ParamMode) -> Result<isize, MachineError> {
        let val = self.get_val(param_mode, 1)?;
        self.pointer_addr += 2;
        Ok(val)
    }

    fn jmpt(
        &mut self,
        param_mode_1: ParamMode,
        param_mode_2: ParamMode,
    ) -> Result<(), MachineError> {
        if self.get_val(param_mode_1, 1)? != 0 {
            let val = self.get_val(param_mode_2, 2)?;
            if val >= 0 {
                self.pointer_addr = val as usize;
            } else {
                return Err(MachineError::InvalidAddress(val));
            }
        } else {
            self.pointer_addr += 3;
        }
        Ok(())
    }

    fn jmpf(
        &mut self,
        param_mode_1: ParamMode,
        param_mode_2: ParamMode,
    ) -> Result<(), MachineError> {
        if self.get_val(param_mode_1, 1)? == 0 {
            let val = self.get_val(param_mode_2, 2)?;
            if val >= 0 {
                self.pointer_addr = val as usize;
            } else {
                return Err(MachineError::InvalidAddress(val));
            }
        } else {
            self.pointer_addr += 3;
        }
        Ok(())
    }

    fn lt(
        &mut self,
        param_mode_1: ParamMode,
        param_mode_2: ParamMode,
        param_mode_3: ParamMode,
    ) -> Result<(), MachineError> {
        let val_1 = self.get_val(param_mode_1, 1)?;
        let val_2 = self.get_val(param_mode_2, 2)?;
        let addr = self.get_addr(param_mode_3, 3)?;
        if val_1 < val_2 {
            self.memory[addr] = 1;
        } else {
            self.memory[addr] = 0;
        }
        self.pointer_addr += 4;
        Ok(())
    }

    fn eq(
        &mut self,
        param_mode_1: ParamMode,
        param_mode_2: ParamMode,
        param_mode_3: ParamMode,
    ) -> Result<(), MachineError> {
        let val_1 = self.get_val(param_mode_1, 1)?;
        let val_2 = self.get_val(param_mode_2, 2)?;
        let addr = self.get_addr(param_mode_3, 3)?;
        if val_1 == val_2 {
            self.memory[addr] = 1;
        } else {
            self.memory[addr] = 0;
        }
        self.pointer_addr += 4;
        Ok(())
    }

    fn adj(&mut self, param_mode: ParamMode) -> Result<(), MachineError> {
        self.relative_base += self.get_val(param_mode, 1)?;
        self.pointer_addr += 2;
        Ok(())
    }

    fn get_val(&self, param_mode: ParamMode, offset: usize) -> Result<isize, MachineError> {
        let mut val = self.memory[self.pointer_addr + offset];
        match param_mode {
            ParamMode::M0 => {
                if self.is_valid_addr(val) {
                    Ok(self.memory[val as usize])
                } else {
                    Err(MachineError::InvalidAddress(val))
                }
            }
            ParamMode::M1 => Ok(val),
            ParamMode::M2 => {
                val += self.relative_base;
                if self.is_valid_addr(val) {
                    Ok(self.memory[val as usize])
                } else {
                    Err(MachineError::InvalidAddress(val))
                }
            }
        }
    }

    fn get_addr(&self, param_mode: ParamMode, offset: usize) -> Result<usize, MachineError> {
        let mut val = self.memory[self.pointer_addr + offset];
        match param_mode {
            ParamMode::M0 => {
                if self.is_valid_addr(val) {
                    Ok(val as usize)
                } else {
                    Err(MachineError::InvalidAddress(val))
                }
            }
            ParamMode::M2 => {
                val += self.relative_base;
                if self.is_valid_addr(val) {
                    Ok(val as usize)
                } else {
                    Err(MachineError::InvalidAddress(val))
                }
            }
            _ => Err(MachineError::InvalidOpcodeError(val.to_string())),
        }
    }

    fn is_valid_addr(&self, val: isize) -> bool {
        val >= 0
    }
}

fn parse_opcode(opcode: isize) -> Result<Opcode, MachineError> {
    if opcode < 0 {
        return Err(MachineError::InvalidOpcodeError(opcode.to_string()));
    }
    let (ins, mut opcode) = (opcode % 100, opcode / 100);
    let mut param = [ParamMode::M0; 3];
    for i in 0..3 {
        param[i] = match opcode % 10 {
            0 => ParamMode::M0,
            1 => ParamMode::M1,
            2 => ParamMode::M2,
            _ => panic!("Invalid parameter mode while parsing!"),
        };
        opcode /= 10;
    }
    match ins {
        1 => Ok(Opcode::ADD(param[0], param[1], param[2])),
        2 => Ok(Opcode::MUL(param[0], param[1], param[2])),
        3 => Ok(Opcode::STR(param[0])),
        4 => Ok(Opcode::LDR(param[0])),
        5 => Ok(Opcode::JMPT(param[0], param[1])),
        6 => Ok(Opcode::JMPF(param[0], param[1])),
        7 => Ok(Opcode::LT(param[0], param[1], param[2])),
        8 => Ok(Opcode::EQ(param[0], param[1], param[2])),
        9 => Ok(Opcode::ADJ(param[0])),
        99 => Ok(Opcode::END),
        _ => Err(MachineError::InvalidOpcodeError(opcode.to_string())),
    }
}
