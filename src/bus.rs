use crate::{framebuffer::Framebuffer, keyboard::Keyboard, ram::RAM};

pub struct Bus {
    ram: RAM,
    keyboard: Keyboard,
    framebuffer: Framebuffer,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            ram: RAM::new(),
            keyboard: Keyboard::new(),
            framebuffer: Framebuffer::new(),
        }
    }

    pub fn read_ram(&self, address: usize) -> u8 {
        self.ram.read(address).unwrap() // TODO: Handle error
    }

    pub fn write_ram(&mut self, data: &[u8], address: usize) {
        self.ram.write(data, address).unwrap(); // TODO: Handle error
    }

    pub fn clear_screen(&mut self) {
        self.framebuffer.clear();
    }
}
