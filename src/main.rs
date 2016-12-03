use std::io;

struct Interpreter {
    memory: Vec<u8>,
    memory_pointer: usize,
    token_pointer: usize,
    input: Box<io::Read>,
    output: Box<io::Write>,
}

impl Interpreter {
    fn new(input: Box<io::Read>, output: Box<io::Write>) -> Interpreter {
        Interpreter {
            memory: vec![0; 20],
            memory_pointer: 0,
            token_pointer: 0,
            input: input,
            output: output,
        }
    }

    fn run(&mut self, tokens: Vec<char>) {
        while let Some(token) = tokens.get(self.token_pointer) {
            match *token {
                '>' => self.memory_pointer += 1,
                '<' => self.memory_pointer -= 1,
                '+' => {
                    if let Some(mut value) = self.memory.get_mut(self.memory_pointer) {
                        *value += 1;
                    }
                }
                '-' => {
                    if let Some(mut value) = self.memory.get_mut(self.memory_pointer) {
                        *value -= 1;
                    }
                }
                '.' => {
                    if let Some(v) = self.memory.get(self.memory_pointer) {
                        self.output.as_mut().write(&[*v; 1]);
                    }
                }
                ',' => {
                    // TODO
                }
                '[' => {
                    if self.memory.get(self.memory_pointer).cloned().unwrap_or(1) == 0 {
                        if let Some(pointer) =
                               self.get_close_bracket_pointer(&tokens, self.token_pointer + 1) {
                            self.token_pointer = pointer + 1;
                            continue;
                        }
                    }
                }
                ']' => {
                    if self.memory.get(self.memory_pointer).cloned().unwrap_or(0) != 0 {
                        if let Some(pointer) =
                               self.get_open_bracket_pointer(&tokens, self.token_pointer - 1) {
                            self.token_pointer = pointer + 1;
                            continue;
                        }
                    }
                }
                _ => {}
            }

            self.token_pointer += 1;
        }
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

fn main() {
    let src: &str = "abc+++abc++++++[>++++++++>++++🍣+++++++>+++++<<<-]>.>++.+++++++..+++.>-.\
                     ------------.<+++++あああ+++.--------.+++.------.--------.>+.";
    let tokens: Vec<char> = src.chars().collect();

    let input = Box::new(io::stdin());
    let output = Box::new(io::stdout());
    let mut interpreter = Interpreter::new(input, output);

    interpreter.run(tokens);
}
