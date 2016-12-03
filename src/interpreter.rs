use std::io;
use super::memory::Memory;

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

    pub fn run(&mut self, src: &str) -> Result<(), io::Error> {
        let tokens: Vec<char> = src.chars().collect();
        while let Some(token) = tokens.get(self.token_pointer) {
            try!(self.step(*token, &tokens));
            self.token_pointer += 1;
        }
        Ok(())
    }

    fn step(&mut self, token: char, tokens: &Vec<char>) -> Result<(), io::Error> {
        match token {
            '>' => self.memory.move_next(),
            '<' => self.memory.move_prev(),
            '+' => self.memory.increment(),
            '-' => self.memory.decrement(),
            '.' => try!(self.output()),
            ',' => try!(self.input()),
            '[' => {
                if self.memory.get() == 0 {
                    if let Some(pointer) =
                           self.get_close_bracket_pointer(&tokens, self.token_pointer + 1) {
                        self.token_pointer = pointer;
                    }
                }
            }
            ']' => {
                if self.memory.get() != 0 {
                    if let Some(pointer) =
                           self.get_open_bracket_pointer(&tokens, self.token_pointer - 1) {
                        self.token_pointer = pointer;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn output(&mut self) -> Result<(), io::Error> {
        let value = self.memory.get();
        self.output.write(&[value]).map(|_| ())
    }

    fn input(&mut self) -> Result<(), io::Error> {
        let mut buffer = [0];
        try!(self.input.read(&mut buffer));
        self.memory.set(buffer[0]);
        Ok(())
    }

    fn get_close_bracket_pointer(&self, tokens: &Vec<char>, start_pointer: usize) -> Option<usize> {
        let mut count = 0;
        let mut pointer = start_pointer;
        while let Some(token) = tokens.get(pointer) {
            match *token {
                ']' => {
                    if count == 0 {
                        return Option::Some(pointer);
                    } else {
                        count -= 1;
                    }
                }
                '[' => count += 1,
                _ => {}
            }
            pointer += 1;
        }
        Option::None
    }

    fn get_open_bracket_pointer(&self, tokens: &Vec<char>, end_pointer: usize) -> Option<usize> {
        let mut count = 0;
        let mut pointer = end_pointer;
        while let Some(token) = tokens.get(pointer) {
            match *token {
                '[' => {
                    if count == 0 {
                        return Option::Some(pointer);
                    } else {
                        count -= 1;
                    }
                }
                ']' => count += 1,
                _ => {}
            }
            pointer -= 1;
        }
        Option::None
    }
}
