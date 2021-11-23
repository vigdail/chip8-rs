use std::path::Path;

use bus::Bus;
use cpu::CPU;

mod bus;
mod cpu;
mod framebuffer;
mod keyboard;
mod ram;

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
        //
    }
}
