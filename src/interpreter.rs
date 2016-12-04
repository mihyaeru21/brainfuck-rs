use std::io;
use super::memory::Memory;
use super::error::Error;

pub struct Interpreter<'a> {
    memory: Memory,
    token_pointer: usize,
    input: &'a mut io::Read,
    output: &'a mut io::Write,
}

impl<'a> Interpreter<'a> {
    pub fn new(memory_size: usize,
               input: &'a mut io::Read,
               output: &'a mut io::Write)
               -> Interpreter<'a> {
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
            '>' => self.memory.move_next()?,
            '<' => self.memory.move_prev()?,
            '+' => self.memory.increment()?,
            '-' => self.memory.decrement()?,
            '.' => self.output()?,
            ',' => self.input()?,
            '[' => self.jump_to_end(&tokens)?,
            ']' => self.jump_to_start(&tokens)?,
            _ => {}
        }
        Ok(())
    }

    fn output(&mut self) -> Result<(), Error> {
        let value = self.memory.get()?;
        self.output.write(&[value]).map(|_| ()).map_err(|e| Error::Io(e))
    }

    fn input(&mut self) -> Result<(), Error> {
        let mut buffer = [0];
        self.input.read(&mut buffer).map_err(|e| Error::Io(e))?;
        self.memory.set(buffer[0])?;
        Ok(())
    }

    fn jump_to_end(&mut self, tokens: &Vec<char>) -> Result<(), Error> {
        if self.memory.get()? == 0 {
            let pointer = self.find_close_bracket_pointer(&tokens, self.token_pointer + 1)?;
            self.token_pointer = pointer;
        }
        Ok(())
    }

    fn jump_to_start(&mut self, tokens: &Vec<char>) -> Result<(), Error> {
        if self.memory.get()? != 0 {
            let pointer = self.find_open_bracket_pointer(&tokens, self.token_pointer - 1)?;
            self.token_pointer = pointer;
        }
        Ok(())
    }

    fn find_close_bracket_pointer(&self,
                                  tokens: &Vec<char>,
                                  start_pointer: usize)
                                  -> Result<usize, Error> {
        let mut count = 0;
        let mut pointer = start_pointer;
        while let Some(token) = tokens.get(pointer) {
            match *token {
                ']' => {
                    if count == 0 {
                        return Ok(pointer);
                    } else {
                        count -= 1;
                    }
                }
                '[' => count += 1,
                _ => {}
            }
            pointer += 1;
        }
        Err(Error::Jump("corresponding bracket is not found."))
    }

    fn find_open_bracket_pointer(&self,
                                 tokens: &Vec<char>,
                                 end_pointer: usize)
                                 -> Result<usize, Error> {
        let mut count = 0;
        let mut pointer = end_pointer;
        while let Some(token) = tokens.get(pointer) {
            match *token {
                '[' => {
                    if count == 0 {
                        return Ok(pointer);
                    } else {
                        count -= 1;
                    }
                }
                ']' => count += 1,
                _ => {}
            }
            pointer -= 1;
        }
        Err(Error::Jump("corresponding bracket is not found."))
    }
}
