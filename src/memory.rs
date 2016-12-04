use super::error::Error;

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

    pub fn move_next(&mut self) -> Result<(), Error> {
        if self.pointer + 1 >= self.bytes.len() {
            Err(Error::Memory("memory pointer overflow."))
        } else {
            self.pointer += 1;
            Ok(())
        }
    }

    pub fn move_prev(&mut self) -> Result<(), Error> {
        if self.pointer < 1 {
            Err(Error::Memory("memory pointer underflow."))
        } else {
            self.pointer -= 1;
            Ok(())
        }
    }

    pub fn increment(&mut self) -> Result<(), Error> {
        let mut value = self.get_mut()?;
        if *value >= 255 {
            Err(Error::Memory("value overflow."))
        } else {
            *value += 1;
            Ok(())
        }
    }

    pub fn decrement(&mut self) -> Result<(), Error> {
        let mut value = self.get_mut()?;
        if *value < 1 {
            Err(Error::Memory("value underflow."))
        } else {
            *value -= 1;
            Ok(())
        }
    }

    pub fn get(&self) -> Result<Byte, Error> {
        self.bytes.get(self.pointer).map(|&v| v).ok_or(Error::Memory("failed to get"))
    }

    pub fn set(&mut self, value: Byte) -> Result<(), Error> {
        let mut v = self.get_mut()?;
        *v = value;
        Ok(())
    }

    fn get_mut(&mut self) -> Result<&mut Byte, Error> {
        self.bytes.get_mut(self.pointer).ok_or(Error::Memory("failed to get"))
    }
}
