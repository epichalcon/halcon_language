mod parser;
mod lex;
#[allow(non_snake_case)]
mod TS;

#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::fs::{File, self};
use std::rc::Rc;

use docopt::Docopt;
use serde::Deserialize;



use crate::parser::{Token, parse};
use crate::TS::TableAdmin;


static USAGE: &'static str = "
Usage: halcon <input> <output> [-c] [-o] [-b] ...

Options:
    -c              : output c file
    -o              : output object file
    -b              : output binary file
";

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Args{
    arg_input: String,
    arg_output: String,
    flag_c: bool,
    flag_o: bool,
    flag_b: bool,
}

pub fn get_args() -> Args{
    Docopt::new(USAGE).and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit())
}

#[allow(unused_variables)]
fn main() -> Result<(), i8> {
    let args: Args = get_args();

    

    let input_file_name = args.arg_input;
    let output_file_name = args.arg_output;



    // se comprueba la validez de los flags

    let file_result = fs::read_to_string(&input_file_name);
    let input_file = match file_result {
        Ok(file) => file ,
        Err(_) => {
            println!("
                Not able to open {}", &input_file_name);
            return Err(3)
        }
    };

    let file_result = File::create(&output_file_name);
    let output_file = match file_result {
        Ok(file) => file ,
        Err(_) => {
            println!("
                Not able to open {}", &output_file_name);
            return Err(3)
        }
    };
    
    let ts = TableAdmin::new();
    
    let mut lexer = lex::Status::new(input_file, ts);

    let mut cont_read = true;

    let mut token_list: Vec<Token> = Vec::new();

    while cont_read {
        let token: Token = lexer.get_token().unwrap();

        if token == Token::Eof{
            cont_read = false;
        }

        token_list.push(token);
    }


    for token in &token_list{
        println!("{:?}", token)
    }
    

    println!("input_file_name: {}", input_file_name);
    println!("output_file_name: {}", output_file_name);
    println!("token len: {}", token_list.len());


    Ok(())
}
