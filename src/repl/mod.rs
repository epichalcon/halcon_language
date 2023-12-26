use std::io::{self, Write};

use crate::ast::Node;
use crate::evaluator::eval;
use crate::lexer::Lexer;
use crate::object::environment::Environment;
use crate::parser::Parser;

const PROMPT: &str = ">>";

pub fn start(input: io::Stdin, mut output: io::Stdout) {
    let mut env = Environment::new();
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

        let (evaluated, new_env) = eval(program, env);

        env = new_env;

        println!("{}", evaluated.inspect());
    }
}
