use std::io::{self, Write};

use crate::lexer::{self, Lexer};
use crate::token::Token;

const PROMPT: &str = ">>";

pub fn start(input: io::Stdin, mut output: io::Stdout) {
    loop {
        print!("{}", PROMPT);
        output.flush().unwrap();
        let mut scanned = String::new();
        let length = input.read_line(&mut scanned).unwrap();
        if length == 1 {
            break;
        }

        let mut lex = Lexer::new(scanned);
        loop {
            let tok = lex.next_token();
            if tok == Token::Eof {
                break;
            }
            println!("{tok}");
        }
    }
}
