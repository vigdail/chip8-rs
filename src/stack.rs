pub const STACK_SIZE: usize = 16;
pub struct Stack {
    data: [u16; STACK_SIZE],
    sp: u8,
}

#[derive(Debug, PartialEq)]
pub enum StackError {
    Overflow,
    OutOfRange,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: [0; STACK_SIZE],
            sp: 0,
        }
    }

    pub fn push(&mut self, item: u16) -> Result<(), StackError> {
        if (self.sp as usize) < STACK_SIZE {
            *self.data.get_mut(self.sp as usize).unwrap() = item;
            self.sp += 1;

            Ok(())
        } else {
            Err(StackError::Overflow)
        }
    }

    pub fn pop(&mut self) -> Result<u16, StackError> {
        if self.sp > 0 {
            self.sp -= 1;
            self.data
                .get(self.sp as usize)
                .cloned()
                .ok_or(StackError::OutOfRange)
        } else {
            Err(StackError::Overflow)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop() {
        let mut stack = Stack::new();
        let expected = 0x1000;
        stack.push(expected).unwrap();
        assert_eq!(Ok(expected), stack.pop());
    }

    #[test]
    fn overflow() {
        let mut stack = Stack::new();
        for i in 0..17 {
            let result = stack.push(0xffff);
            if i < STACK_SIZE {
                assert!(result.is_ok());
            } else {
                assert!(result.is_err());
            }
        }
    }

    #[test]
    fn pop_empty() {
        let mut stack = Stack::new();
        assert_eq!(stack.pop(), Err(StackError::Overflow));
    }
}
