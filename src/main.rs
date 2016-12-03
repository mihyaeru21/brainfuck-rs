extern crate brainfuck;

use std::io;
use brainfuck::interpreter::Interpreter;

fn main() {
    let src: &str = "abc+++abc++++++[>++++++++>++++🍣+++++++>+++++<<<-]>.>++.+++++++..+++.>-.\
                     ------------.<+++++あああ+++.--------.+++.------.--------.>+.,.";
    let tokens: Vec<char> = src.chars().collect();

    let input = Box::new(io::stdin());
    let output = Box::new(io::stdout());
    let mut interpreter = Interpreter::new(20, input, output);

    if let Err(e) = interpreter.run(tokens) {
        println!("{}", e);
    }
}
