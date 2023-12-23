use std::io::{self, Write};

use crate::ast::Node;
use crate::lexer::Lexer;
use crate::parser::Parser;

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

        let lex = Lexer::new(scanned);
        let mut pars = Parser::new(lex);

        let program = pars.parse_program();

        if pars.errors().len() != 0 {
            println!("Errors have been found: \n{:?}", pars.errors());
            continue;
        }

        println!("{}", program.string());
    }
}
