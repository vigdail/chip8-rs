const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Framebuffer {
    buffer: [u8; WIDTH * HEIGHT],
}

impl Framebuffer {
    pub fn new() -> Self {
        Self {
            buffer: [0; WIDTH * HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [0; WIDTH * HEIGHT];
    }

    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn draw(&mut self, x: u8, y: u8, byte: u8) -> bool {
        let mut has_collision = false;
        let mut x = x as usize;
        let y = y as usize % HEIGHT;
        let mut byte = byte;

        for _ in 0..8 {
            x %= WIDTH;
            let index = Framebuffer::xy_to_index(x, y);

            let bit = (byte & 0x80) >> 7;

            if self.buffer[index] == 1 && bit == 0 {
                has_collision = true;
            }

            self.buffer[index] ^= bit;

            x += 1;
            byte <<= 1;
        }

        has_collision
    }

    fn xy_to_index(x: usize, y: usize) -> usize {
        y * WIDTH + x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_byte() {
        let mut framebuffer = Framebuffer::new();

        let byte = 0b01010101;
        let x = 0;
        let y = 5;
        let index = Framebuffer::xy_to_index(x as usize, y as usize);
        let expected = &[0, 1, 0, 1, 0, 1, 0, 1];

        framebuffer.draw(x, y, byte);
        let actual = &framebuffer.buffer[index..index + 8];

        assert_eq!(actual, expected);
    }

    #[test]
    fn collision() {
        let mut framebuffer = Framebuffer::new();

        let byte1 = 0b11100001;
        let byte2 = 0b00000001;
        let x = 0;
        let y = 5;

        let has_collision = framebuffer.draw(x, y, byte1);
        assert!(!has_collision);

        let has_collision = framebuffer.draw(x, y, byte2);
        assert!(has_collision);
    }

    #[test]
    fn wrapping_x() {
        let mut framebuffer = Framebuffer::new();

        let byte = 0b11100001;
        let x = (WIDTH - 4) as u8;
        let y = 0;

        framebuffer.draw(x, y, byte);
        let start = &framebuffer.buffer[..4];
        let end = &framebuffer.buffer[60..64];
        assert_eq!(start, &[0, 0, 0, 1]);
        assert_eq!(end, &[1, 1, 1, 0]);
    }

    #[test]
    fn wrapping_y() {
        let mut framebuffer = Framebuffer::new();

        let byte = 0b11100001;
        let x = 0;
        let y = (HEIGHT + 1) as u8;
        let index = Framebuffer::xy_to_index(0, 1);

        framebuffer.draw(x, y, byte);
        let actual = &framebuffer.buffer[index..index + 8];
        assert_eq!(actual, &[1, 1, 1, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn xy_to_index() {
        let x = 4;
        let y = 20;
        let expected = 1284;
        let actual = Framebuffer::xy_to_index(x, y);

        assert_eq!(actual, expected);
    }
}
