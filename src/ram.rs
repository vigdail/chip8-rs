pub const RAM_SIZE: usize = 4096;

#[derive(Debug)]
pub enum RamError {
    BadWriteAddress,
    BadReadAddress,
}

pub struct Ram {
    memory: [u8; RAM_SIZE],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            memory: [0; RAM_SIZE],
        }
    }

    pub fn write(&mut self, data: &[u8], address: usize) -> Result<(), RamError> {
        if address + data.len() >= RAM_SIZE {
            return Err(RamError::BadWriteAddress);
        }

        dbg!(address, data);
        for (offset, &byte) in data.iter().enumerate() {
            self.memory[address + offset] = byte;
        }

        Ok(())
    }

    pub fn read(&self, address: usize) -> Result<u8, RamError> {
        if address >= RAM_SIZE {
            return Err(RamError::BadReadAddress);
        }

        Ok(self.memory[address])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write() {
        let mut ram = Ram::new();

        let result = ram.write(&[0xff, 0xfe], 0);
        assert!(result.is_ok());
    }

    #[test]
    fn read_byte() {
        let mut ram = Ram::new();

        ram.write(&[0xff, 0xfe], 0).unwrap();

        let actual_0 = ram.read(0).unwrap();
        let actual_1 = ram.read(1).unwrap();

        assert_eq!(actual_0, 0xff);
        assert_eq!(actual_1, 0xfe);
    }
}
