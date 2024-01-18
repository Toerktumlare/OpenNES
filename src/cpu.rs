#![allow(dead_code, clippy::upper_case_acronyms)]

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: &[u8]) {
        self.program_counter = 0;

        loop {
            let opcode = self.next_opcode(program);

            match opcode {
                Opcode::LDA(param) => {
                    self.set_register(Register::A, param);
                }
                Opcode::TAX => self.set_register(Register::X, self.register_a),
                Opcode::INX => self.inc_register(Register::X),
                Opcode::BRK => {
                    break;
                }
                Opcode::Unknown(value) => unimplemented!("Opcode: 0x{:X}", value),
            }
        }
    }

    fn next_opcode(&mut self, program: &[u8]) -> Opcode {
        let opcode = program[self.program_counter as usize];
        self.program_counter += 1;
        match opcode {
            0x00 => Opcode::BRK,
            0xA9 => {
                let opcode = Opcode::LDA(program[self.program_counter as usize]);
                self.program_counter += 1;
                opcode
            }
            0xAA => Opcode::TAX,
            0xE8 => Opcode::INX,
            value => Opcode::Unknown(value),
        }
    }

    fn set_register(&mut self, register: Register, param: u8) {
        match register {
            Register::A => {
                self.register_a = param;

                if self.register_a == 0 {
                    self.set_flag(Flag::Zero, true);
                } else {
                    self.set_flag(Flag::Zero, false);
                }
                if self.register_a & 0b1000_0000 != 0 {
                    self.set_flag(Flag::Negative, true);
                } else {
                    self.set_flag(Flag::Negative, false);
                }
            }
            Register::X => {
                self.register_x = param;
                if self.register_x == 0 {
                    self.set_flag(Flag::Zero, true);
                } else {
                    self.set_flag(Flag::Zero, false);
                }
                if self.register_x & 0b1000_0000 != 0 {
                    self.set_flag(Flag::Negative, true);
                } else {
                    self.set_flag(Flag::Negative, false);
                }
            }
        }
    }

    fn inc_register(&mut self, register: Register) {
        match register {
            Register::A => self.set_register(Register::A, self.register_a + 1),
            Register::X => self.set_register(Register::X, self.register_a + 1),
        }
    }

    fn get_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::Zero => self.status & 0b0000_0010 != 0b00,
            Flag::Negative => self.status & 0b1000_0000 != 0,
        }
    }

    fn set_flag(&mut self, flag: Flag, bool: bool) {
        match flag {
            Flag::Negative => {
                if bool {
                    self.status |= 0b1000_0000;
                } else {
                    self.status &= 0b0111_1111;
                }
            }
            Flag::Zero => {
                if bool {
                    self.status |= 0b0000_0010;
                } else {
                    self.status &= 0b1111_1101;
                }
            }
        }
    }
}

enum Flag {
    Negative,
    Zero,
}

enum Opcode {
    BRK,     // 0x00
    LDA(u8), // 0xA9
    TAX,     // 0xAA
    INX,
    Unknown(u8),
}

enum Register {
    A,
    X,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(&[0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(&[0xa9, 0x00, 0x00]);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(&[0xAA, 0x00]);

        assert_eq!(cpu.register_x, 0xA);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(&[0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(&[0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}
