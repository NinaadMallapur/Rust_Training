pub struct CircularBuffer {
    array: [i32; 3],
    head: i32,
    tail: i32,
}

#[derive(Debug)]
pub enum Errors {
    BufferFull,
    BufferEmpty,
}

impl CircularBuffer {
    fn new() -> Self {
        CircularBuffer {
            array: [0; 3],
            head: -1,
            tail: -1,
        }
    }

    fn push(&mut self, value: i32) -> Result<(), Errors> {
        if self.is_full() {
            return Err(Errors::BufferFull);
        }
        self.head = self.head + 1;
        self.array[self.head as usize] = value;

        Ok(())
    }

    fn is_full(&mut self) -> bool {
        return self.head + 1 >= self.array.len() as i32;
    }
}

fn main() {}

mod tests {
    use crate::CircularBuffer;
    use crate::Errors;

    #[test]
    fn test_push() {
        let mut cb = CircularBuffer::new();

        let _ = cb.push(9);
        assert_eq!(cb.array[0], 9);

        let _ = cb.push(99);
        assert_eq!(cb.array[1], 99);

        let _ = cb.push(102);
        assert_eq!(cb.array[2], 102);

        let return_code = cb.push(222);
        assert!(matches!(return_code, Err(Errors::BufferFull)));
    }
}
