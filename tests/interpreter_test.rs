extern crate brainfuck;

use brainfuck::interpreter::Interpreter;

#[test]
fn run() {
    let mut input = "🍣".as_bytes();
    let mut output: Vec<u8> = vec![];

    let src = "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-.\
                     ------------.<++++++++.--------.+++.------.--------.>+.,.,.,.,.";

    let _ = Interpreter::new(20, &mut input, &mut output).run(&src);

    let output_str = std::str::from_utf8(&output).unwrap();
    assert_eq!(output_str, "Hello, world!🍣");
}
