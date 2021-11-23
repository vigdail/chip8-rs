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

    pub fn write_ram(&mut self, data: &[u8], offset: usize) {
        self.ram.write(data, offset);
    }
}
