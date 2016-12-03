type Byte = u8;

pub struct Memory {
    bytes: Vec<Byte>,
    pointer: usize,
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        Memory {
            bytes: vec![0; size],
            pointer: 0,
        }
    }

    pub fn move_next(&mut self) {
        self.pointer += 1;
    }

    pub fn move_prev(&mut self) {
        self.pointer -= 1;
    }

    pub fn increment(&mut self) {
        self.bytes[self.pointer] += 1;
    }

    pub fn decrement(&mut self) {
        self.bytes[self.pointer] -= 1;
    }

    pub fn get(&self) -> Byte {
        self.bytes.get(self.pointer).cloned().unwrap()
    }

    pub fn set(&mut self, value: Byte) {
        self.bytes[self.pointer] = value;
    }
}
