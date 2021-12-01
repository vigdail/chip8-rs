use bus::Bus;
use cpu::Cpu;
use std::path::Path;

mod bus;
mod cpu;
mod framebuffer;
mod keyboard;
mod ram;
mod stack;

pub const ENTRY_POINT: u16 = 0x200;

pub struct Chip8 {
    bus: Bus,
    cpu: Cpu,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            bus: Bus::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn load_program<P: AsRef<Path>>(&mut self, path: P) {
        let data = std::fs::read(path).unwrap(); // TODO: Handle error
        self.set_program(data.as_slice());
    }

    pub fn set_program(&mut self, data: &[u8]) {
        self.bus.write_ram(data, ENTRY_POINT);
    }

    pub fn get_framebuffer(&self) -> &[u8] {
        self.bus.get_framebuffer()
    }

    pub fn set_key_pressed(&mut self, key: Option<u8>) {
        self.bus.set_key_pressed(key);
    }

    pub fn run(&mut self) {
        self.cpu.run(&mut self.bus);
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}
