use std::io;
use super::memory::Memory;
use super::error::Error;

pub struct Interpreter<R, W> {
    memory: Memory,
    token_pointer: usize,
    input: R,
    output: W,
}

impl<R: io::Read, W: io::Write> Interpreter<R, W> {
    pub fn new(memory_size: usize, input: R, output: W) -> Interpreter<R, W> {
        Interpreter {
            memory: Memory::new(memory_size),
            token_pointer: 0,
            input: input,
            output: output,
        }
    }

    pub fn run(&mut self, src: &str) -> Result<(), Error> {
        let tokens: Vec<char> = src.chars().collect();
        while let Some(token) = tokens.get(self.token_pointer) {
            self.step(*token, &tokens)?;
            self.token_pointer += 1;
        }
        Ok(())
    }

    fn step(&mut self, token: char, tokens: &Vec<char>) -> Result<(), Error> {
        match token {
            '>' => self.memory.move_next(),
            '<' => self.memory.move_prev(),
            '+' => self.memory.increment(),
            '-' => self.memory.decrement(),
            '.' => self.output(),
            ',' => self.input(),
            '[' => self.jump_to_end(&tokens),
            ']' => self.jump_to_start(&tokens),
            _ => Ok(()),
        }
    }

    fn output(&mut self) -> Result<(), Error> {
        let value = self.memory.get()?;
        self.output.write_all(&[value])?;
        Ok(())
    }

    fn input(&mut self) -> Result<(), Error> {
        let mut buffer = [0];
        self.input.read_exact(&mut buffer)?;
        self.memory.set(buffer[0])
    }

    fn jump_to_end(&mut self, tokens: &Vec<char>) -> Result<(), Error> {
        if self.memory.get()? == 0 {
            let pointer = self.find_close_pointer(&tokens, self.token_pointer + 1, 0)?;
            self.token_pointer = pointer;
        }
        Ok(())
    }

    fn jump_to_start(&mut self, tokens: &Vec<char>) -> Result<(), Error> {
        if self.memory.get()? != 0 {
            let pointer = self.find_open_pointer(&tokens, self.token_pointer - 1, 0)?;
            self.token_pointer = pointer;
        }
        Ok(())
    }

    fn find_close_pointer(&self,
                          tokens: &Vec<char>,
                          start_pointer: usize,
                          count: usize)
                          -> Result<usize, Error> {
        for pointer in start_pointer..tokens.len() {
            match (tokens[pointer], count) {
                (']', 0) => return Ok(pointer),
                (']', _) => return self.find_close_pointer(tokens, start_pointer, count - 1),
                ('[', _) => return self.find_close_pointer(tokens, start_pointer, count + 1),
                _ => {}
            }
        }
        Err(Error::Jump("corresponding bracket is not found."))
    }

    fn find_open_pointer(&self,
                         tokens: &Vec<char>,
                         end_pointer: usize,
                         count: usize)
                         -> Result<usize, Error> {
        for pointer in (0..end_pointer).rev() {
            match (tokens[pointer], count) {
                ('[', 0) => return Ok(pointer),
                ('[', _) => return self.find_open_pointer(tokens, end_pointer, count - 1),
                (']', _) => return self.find_open_pointer(tokens, end_pointer, count + 1),
                _ => {}
            }
        }
        Err(Error::Jump("corresponding bracket is not found."))
    }
}
