use bus::Bus;
use cpu::CPU;
use std::path::Path;

mod bus;
mod cpu;
mod framebuffer;
mod keyboard;
mod ram;

pub const ENTRY_POINT: usize = 0x200;

pub struct Chip8 {
    bus: Bus,
    cpu: CPU,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            bus: Bus::new(),
            cpu: CPU::new(),
        }
    }

    pub fn load_program<P: AsRef<Path>>(&mut self, path: P) {
        let data = std::fs::read(path).unwrap(); // TODO
        self.set_program(data.as_slice());
    }

    pub fn set_program(&mut self, data: &[u8]) {
        self.bus.write_ram(data, ENTRY_POINT);
    }
}
