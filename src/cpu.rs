use crate::bus::Bus;

pub struct CPU {
    vx: [u8; 16],
    i: u16,
    delay_timer: u16,
    sound_timer: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
}

impl CPU {
    pub fn new() -> Self {
        Self {
            vx: [0; 16],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0,
            sp: 0,
            stack: [0; 16],
        }
    }

    pub fn execute(&mut self, bus: &mut Bus) {
        //
    }
}
