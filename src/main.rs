pub struct CircularBuffer {
    array: [i32; 3],
    head: i32,
    tail: i32,
}

impl CircularBuffer {
    fn new() -> Self {
        CircularBuffer {
            array: [0; 3],
            head: -1,
            tail: -1,
        }
    }
}

fn main() {
    let mut cb = CircularBuffer::new();
}
