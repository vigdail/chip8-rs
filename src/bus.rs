#![allow(dead_code)]
use crate::{framebuffer::Framebuffer, keyboard::Keyboard, ram::Ram};

pub struct Bus {
    ram: Ram,
    keyboard: Keyboard,
    framebuffer: Framebuffer,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            ram: Ram::new(),
            keyboard: Keyboard::new(),
            framebuffer: Framebuffer::new(),
        }
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        self.ram.read(address as usize).unwrap() // TODO: Handle error
    }

    pub fn write_ram(&mut self, data: &[u8], address: u16) {
        self.ram.write(data, address as usize).unwrap(); // TODO: Handle error
    }

    pub fn clear_screen(&mut self) {
        self.framebuffer.clear();
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.keyboard.get_key_pressed()
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keyboard.is_key_pressed(key)
    }

    pub fn set_key_pressed(&mut self, key: Option<u8>) {
        self.keyboard.set_key_pressed(key)
    }

    pub fn draw(&mut self, _x: u8, _y: u8, _data: u8) -> bool {
        todo!()
    }
}
