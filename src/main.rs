mod ast;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;
mod token;

use std::{fs, io};

use docopt::Docopt;
use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;
use serde::Deserialize;

static USAGE: &'static str = "
Usage: halcon [<input>]

    default: runns the language REPL
    Option:
        <input>: an input file can be specified from whitch to get the code
";

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Args {
    arg_input: Option<String>,
}

pub fn get_args() -> Args {
    Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit())
}

fn main() {
    let args: Args = get_args();

    match args.arg_input {
        Some(input_file_name) => {
            if !input_file_name.ends_with(".hc") {
                panic!("not a valid extension")
            }
            execute_file(input_file_name);
        }
        None => {
            println!("Welcome to halcon");
            repl::start(io::stdin(), io::stdout());
        }
    }
}

fn execute_file(file_name: String) {
    let contents = fs::read_to_string(file_name).unwrap();
    let lex = Lexer::new(contents);
    let mut pars = Parser::new(lex);

    let program = pars.parse_program();

    if pars.errors().len() != 0 {
        println!("Errors have been found: \n{:?}", pars.errors());
        return;
    }

    let mut evaluator = Evaluator::new();
    let _ = evaluator.eval(program);
}
