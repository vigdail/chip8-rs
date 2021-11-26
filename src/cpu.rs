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

#[derive(Debug)]
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
        println!("Params: {:?}", params);
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
                // LD vx, byte
                self.write_reg(params.x, params.kk);
                self.pc += 2;
            }
            0x7 => {
                // ADD vx, byte
                self.write_reg(params.x, self.vx[params.x as usize].wrapping_add(params.kk));
                self.pc += 2;
            }
            0x8 => {
                let vx = self.vx[params.x as usize];
                let vy = self.vx[params.y as usize];
                match params.n {
                    0x0 => {
                        // LD vx, vy
                        self.write_reg(params.x, vy);
                    }
                    0x1 => {
                        // OR vx, vy
                        println!("vx = {:2x?}, vy = {:2x?}, or = {:2x?}", vx, vy, vx | vy);
                        self.write_reg(params.x, vx | vy);
                    }
                    0x2 => {
                        // AND vx, vy
                        self.write_reg(params.x, vx & vy);
                    }
                    0x3 => {
                        // XOR vx, vy
                        self.write_reg(params.x, vx ^ vy);
                    }
                    0x4 => {
                        // ADD vx, vy
                        let sum = vx as u16 + vy as u16;
                        self.write_reg(params.x, sum as u8);
                        if sum > 0xff {
                            self.write_reg(0xf, 1);
                        }
                    }
                    0x5 => {
                        // SUB vx, vy
                        if vx > vy {
                            self.write_reg(0xf, 1);
                        } else {
                            self.write_reg(0xf, 0);
                        }

                        self.write_reg(params.x, vx.wrapping_sub(vy));
                    }
                    0x6 => {
                        // SHR vx
                        self.write_reg(0xf, vx & 0x1);
                        self.write_reg(params.x, vx >> 1);
                    }
                    0x7 => {
                        // SUBN vx, vy
                        if vy > vx {
                            self.write_reg(0xf, 1);
                        } else {
                            self.write_reg(0xf, 0);
                        }

                        self.write_reg(params.x, vy.wrapping_sub(vx));
                    }
                    0xe => {
                        // SHL vx
                        self.write_reg(0xf, (vx & 0x80) >> 7);
                        self.write_reg(params.x, vx << 1);
                    }
                    _ => {}
                }

                self.pc += 2;
            }
            0x9 => {
                // SNE vx, vy
                let vx = self.vx[params.x as usize];
                let vy = self.vx[params.y as usize];

                if vx != vy {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            _ => todo!(),
        }
    }

    fn write_reg(&mut self, x: u8, value: u8) {
        self.vx[x as usize] = value;
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
                0x91, 0x50, // 0x0214: sne v1, v2
                0x61, 0xff, // 0x0212: ld v1 0xff
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

        cpu.run(&mut bus);
        cpu.run(&mut bus);
        assert_eq!(cpu.pc, 0x0218);
        assert_eq!(cpu.vx[1], 0xff);
        assert_eq!(cpu.vx[5], 0xc1);
    }

    #[test]
    fn inst_0x7xkk() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();

        bus.write_ram(
            &[
                0x71, 0x20, // 0x0200: add v1 0x20
                0x71, 0x20, // 0x0202: add v1 0x20
                0x71, 0xE0, // 0x0204: add v1 0xe0
                0x72, 0x01, // 0x0206: add v2 0x01
            ],
            ENTRY_POINT,
        );

        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0x20);
        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0x40);
        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0x20);
        cpu.run(&mut bus);
        assert_eq!(cpu.vx[2], 0x01);
    }

    #[test]
    fn inst_0x8xyn() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();

        bus.write_ram(
            &[
                0x61, 0xF4, // 0x0200: ld v1, 0xf4  (v1 = 0b11110100)
                0x62, 0x82, // 0x0202: ld v2, 0x82  (v2 = 0b10000010)
                0x81, 0x21, // 0x0204: or v1, v2    (v1 = 0b11110110)
                0x81, 0x22, // 0x0206: and v1, v2   (v1 = 0b10000010)
                0x62, 0x11, // 0x0208: ld v2, 0x11  (v2 = 0b00010001)
                0x81, 0x20, // 0x020a: ld v1, v2    (v1 = 0b00010001)
                0x62, 0x85, // 0x020c: ld v2, 0x85  (v2 = 0b10000101)
                0x81, 0x23, // 0x020e: xor v1, v2   (v1 = 0b10010100)
                0x81, 0x24, // 0x0210: add v1, v2   (v1 = 0b00011001)
                0x81, 0x24, // 0x0212: add v1, v2   (v1 = 0b10011110)
                0x81, 0x25, // 0x0214: sub v1, v2   (v1 = 0b00011001)
                0x81, 0x25, // 0x0216: sub v1, v2   (v1 = 0b10010100)
                0x81, 0x06, // 0x0218: shr v1       (v1 = 0b01001010)
                0x82, 0x06, // 0x021a: shr v2       (v2 = 0b01000010)
                0x81, 0x27, // 0x021c: subn v1, v2  (v1 = 0b11111000)
                0x81, 0x0e, // 0x021e: shl v1       (v1 = 0b11110000)
                0x81, 0x06, // 0x0220: shr v1       (v1 = 0b01111000)
                0x81, 0x0e, // 0x0222: shl v1       (v1 = 0b11110000)
            ],
            ENTRY_POINT,
        );

        cpu.run(&mut bus);
        cpu.run(&mut bus);
        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0xf6);
        assert_eq!(cpu.vx[2], 0x82);

        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0x82);
        assert_eq!(cpu.vx[2], 0x82);

        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0x82);
        assert_eq!(cpu.vx[2], 0x11);
        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0x11);
        assert_eq!(cpu.vx[2], 0x11);

        cpu.run(&mut bus);
        assert_eq!(cpu.vx[2], 0x85);
        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0x94);
        assert_eq!(cpu.vx[0xf], 0x0);

        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0x19);
        assert_eq!(cpu.vx[2], 0x85);
        assert_eq!(cpu.vx[0xf], 0x1);

        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0x9e);
        assert_eq!(cpu.vx[2], 0x85);

        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0x19);
        assert_eq!(cpu.vx[2], 0x85);
        assert_eq!(cpu.vx[0xf], 0x1);

        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0x94);
        assert_eq!(cpu.vx[2], 0x85);
        assert_eq!(cpu.vx[0xf], 0x0);

        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0x4a);
        assert_eq!(cpu.vx[2], 0x85);
        assert_eq!(cpu.vx[0xf], 0x0);

        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0x4a);
        assert_eq!(cpu.vx[2], 0x42);
        assert_eq!(cpu.vx[0xf], 0x1);

        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0xf8);
        assert_eq!(cpu.vx[2], 0x42);
        assert_eq!(cpu.vx[0xf], 0x0);

        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0xf0);
        assert_eq!(cpu.vx[2], 0x42);
        assert_eq!(cpu.vx[0xf], 0x1);

        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0x78);
        assert_eq!(cpu.vx[2], 0x42);
        assert_eq!(cpu.vx[0xf], 0x0);

        cpu.run(&mut bus);
        assert_eq!(cpu.vx[1], 0xf0);
        assert_eq!(cpu.vx[2], 0x42);
        assert_eq!(cpu.vx[0xf], 0x0);
    }
}
