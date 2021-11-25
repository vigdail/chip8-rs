#![allow(dead_code)]
use crate::{bus::Bus, stack::Stack, ENTRY_POINT};

pub struct Cpu {
    vx: [u8; 16],
    i: u16,
    delay_timer: u16,
    sound_timer: u16,
    pc: u16,
    stack: Stack,
}

struct InstructionData {
    instruction: u16,
    nnn: u16,
    kk: u8,
    x: u8,
    y: u8,
    n: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            vx: [0; 16],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: ENTRY_POINT as u16,
            stack: Stack::new(),
        }
    }

    pub fn run(&mut self, bus: &mut Bus) {
        let instruction = self.fetch_instruction(bus);
        let params = Self::parse_instruction(instruction);
        self.execute(bus, params);
    }

    fn execute(&mut self, bus: &mut Bus, params: InstructionData) {
        let f = (params.instruction & 0xf000) >> 12;
        match f {
            0x0 => match params.kk {
                0x0e => {
                    // CLS
                    bus.clear_screen();
                    self.pc += 2;
                }
                0xee => {
                    // RET
                    let addr = self.stack.pop().unwrap(); // TODO: Handle error
                    self.pc = addr;
                }
                _ => unimplemented!(),
            },
            0x1 => {
                // JP nnn
                self.pc = params.nnn;
            }
            0x2 => {
                // CALL nnn
                self.stack.push(self.pc + 2).unwrap(); // TODO: Handle error
                self.pc = params.nnn;
            }
            0x3 => {
                // SE vx, byte
                if self.vx[params.x as usize] == params.kk {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x4 => {
                // SNE vx, byte
                if self.vx[params.x as usize] != params.kk {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x5 => {
                // SE vx, vy
                if self.vx[params.x as usize] == self.vx[params.y as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x6 => {
                // LD vx, kk
                self.vx[params.x as usize] = params.kk;
                self.pc += 2;
            }
            _ => todo!(),
        }
    }

    fn fetch_instruction(&mut self, bus: &mut Bus) -> u16 {
        let hi = bus.read_ram(self.pc as usize) as u16;
        let lo = bus.read_ram((self.pc + 1) as usize) as u16;

        hi << 8 | lo
    }

    fn parse_instruction(instruction: u16) -> InstructionData {
        let nnn = instruction & 0x0fff;
        let kk = (instruction & 0x00ff) as u8;
        let x = ((instruction & 0x0f00) >> 8) as u8;
        let y = ((instruction & 0x00f0) >> 4) as u8;
        let n = (instruction & 0x000f) as u8;

        InstructionData {
            instruction,
            nnn,
            kk,
            x,
            y,
            n,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_instruction() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        bus.write_ram(&[0x00, 0xE0], 0x200);

        let instruction = cpu.fetch_instruction(&mut bus);

        assert_eq!(instruction, 0x00e0);
    }

    #[test]
    fn parse_instruction() {
        let params = Cpu::parse_instruction(0x1234);

        assert_eq!(params.instruction, 0x1234);
        assert_eq!(params.nnn, 0x0234);
        assert_eq!(params.kk, 0x34);
        assert_eq!(params.x, 0x2);
        assert_eq!(params.y, 0x3);
        assert_eq!(params.n, 0x4);
    }

    #[test]
    fn subroutine() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        bus.write_ram(
            &[
                0x12, 0x04, // 0x0200: jmp 204
                0x00, 0xee, // 0x0202: ret
                0x22, 0x02, // 0x0204: call 202
            ],
            ENTRY_POINT,
        );

        assert_eq!(cpu.pc, 0x0200);
        cpu.run(&mut bus);
        assert_eq!(cpu.pc, 0x0204);
        cpu.run(&mut bus);
        assert_eq!(cpu.pc, 0x0202);
        cpu.run(&mut bus);
        assert_eq!(cpu.pc, 0x0206);
    }

    #[test]
    fn skips() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        bus.write_ram(
            &[
                0x31, 0x01, // 0x0200: se v1 0x01
                0x61, 0x0f, // 0x0202: ld v1 0x0f
                0x31, 0x0f, // 0x0204: se v1 0x0f
                0x00, 0x00, // 0x0206: illegal, should be skipped
                0x65, 0xc1, // 0x0208: ld v5 0xc1
                0x61, 0xc1, // 0x020a: ld v1 0xc1
                0x51, 0x50, // 0x020c: se v1 v5
                0x00, 0x00, // 0x020e: illegal, should be skipped
                0x41, 0x50, // 0x0210: sne v1 0x50
                0x00, 0x00, // 0x0212: illegal, should be skipped
            ],
            ENTRY_POINT,
        );

        assert_eq!(cpu.pc, 0x0200);

        cpu.run(&mut bus);
        assert_eq!(cpu.pc, 0x0202);

        cpu.run(&mut bus);
        assert_eq!(cpu.pc, 0x0204);
        assert_eq!(cpu.vx[1], 0x0f);

        cpu.run(&mut bus);
        assert_eq!(cpu.pc, 0x0208);
        assert_eq!(cpu.vx[1], 0x0f);

        cpu.run(&mut bus);
        assert_eq!(cpu.pc, 0x020a);
        assert_eq!(cpu.vx[1], 0x0f);
        assert_eq!(cpu.vx[5], 0xc1);

        cpu.run(&mut bus);
        assert_eq!(cpu.pc, 0x020c);
        assert_eq!(cpu.vx[1], 0xc1);
        assert_eq!(cpu.vx[5], 0xc1);

        cpu.run(&mut bus);
        assert_eq!(cpu.pc, 0x0210);
        assert_eq!(cpu.vx[1], 0xc1);
        assert_eq!(cpu.vx[5], 0xc1);

        cpu.run(&mut bus);
        assert_eq!(cpu.pc, 0x0214);
        assert_eq!(cpu.vx[1], 0xc1);
        assert_eq!(cpu.vx[5], 0xc1);
    }
}
