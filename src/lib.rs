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
        let mut bus = Bus::new();
        bus.write_ram(
            &[
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xF0, 0x80, 0x80, // F
            ],
            0x0,
        );

        Self {
            bus,
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

    pub fn tick_timers(&mut self) {
        self.cpu.tick_timers();
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}
