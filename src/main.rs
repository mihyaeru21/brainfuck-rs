extern crate brainfuck;

use std::io;
use brainfuck::interpreter::Interpreter;

fn main() {
    let src = "abc+++abc++++++[>++++++++>++++🍣+++++++>+++++<<<-]>.>++.+++++++..+++.>-.\
                     ------------.<+++++あああ+++.--------.+++.------.--------.>+.";

    let mut input = io::stdin();
    let mut output = io::stdout();
    let mut interpreter = Interpreter::new(30000, &mut input, &mut output);

    if let Err(e) = interpreter.run(src) {
        println!("{}", e);
    }
}
