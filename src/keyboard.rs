pub struct Keyboard {
    key_pressed: Option<u8>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self { key_pressed: None }
    }

    pub fn set_key_pressed(&mut self, key: Option<u8>) {
        self.key_pressed = key;
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.key_pressed
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.key_pressed.map_or(false, |pressed| pressed == key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_key_pressed() {
        let expected = Some(4);
        let keyboard = Keyboard {
            key_pressed: expected,
        };

        let actual = keyboard.get_key_pressed();
        assert_eq!(actual, expected);
    }

    #[test]
    fn set_key_pressed() {
        let mut keyboard = Keyboard::new();
        assert_eq!(keyboard.get_key_pressed(), None);

        let expected = Some(4);
        keyboard.set_key_pressed(expected);
        let actual = keyboard.get_key_pressed();
        assert_eq!(actual, expected);
    }

    #[test]
    fn is_key_pressed() {
        let keyboard = Keyboard {
            key_pressed: Some(2),
        };

        assert!(keyboard.is_key_pressed(2));
        assert!(!keyboard.is_key_pressed(5));
    }
}
