fn main() {
    let src: &str = "abc+++abc++++++[>++++++++>++++🍣+++++++>+++++<<<-]>.>++.+++++++..+++.>-.\
                     ------------.<+++++あああ+++.--------.+++.------.--------.>+.";
    let tokens: Vec<char> = src.chars().collect();

    let mut memory: Vec<u32> = vec![0; 20];
    let mut memory_pointer: usize = 0;
    let mut token_pointer: usize = 0;

    while let Some(token) = tokens.get(token_pointer) {
        match *token {
            '>' => memory_pointer += 1,
            '<' => memory_pointer -= 1,
            '+' => {
                if let Some(mut value) = memory.get_mut(memory_pointer) {
                    *value += 1;
                }
            }
            '-' => {
                if let Some(mut value) = memory.get_mut(memory_pointer) {
                    *value -= 1;
                }
            }
            '.' => {
                if let Some(c) = memory.get(memory_pointer)
                    .and_then(|v| std::char::from_u32(*v)) {
                    print!("{}", c);
                }
            }
            ',' => {
                // TODO
            }
            '[' => {
                if memory.get(memory_pointer).cloned().unwrap_or(1) == 0 {
                    if let Some(pointer) = get_close_bracket_pointer(&tokens, token_pointer + 1) {
                        token_pointer = pointer + 1;
                        continue;
                    }
                }
            }
            ']' => {
                if memory.get(memory_pointer).cloned().unwrap_or(0) != 0 {
                    if let Some(pointer) = get_open_bracket_pointer(&tokens, token_pointer - 1) {
                        token_pointer = pointer + 1;
                        continue;
                    }
                }
            }
            _ => {}
        }

        token_pointer += 1;
    }
}

fn get_close_bracket_pointer(tokens: &Vec<char>, start_pointer: usize) -> Option<usize> {
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

fn get_open_bracket_pointer(tokens: &Vec<char>, end_pointer: usize) -> Option<usize> {
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
