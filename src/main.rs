mod lex;
mod TS;

#[cfg(test)]
mod tests;

use std::fs::{File, self};

use docopt::Docopt;
use serde::Deserialize;



use crate::TS::TableAdmin;


static USAGE: &'static str = "
Usage: halcon <input> <output> [-c] [-o] [-b] ...

Options:
    -c              : output c file
    -o              : output object file
    -b              : output binary file
";

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
    
    let mut ts = TableAdmin::new();
    
    let mut lexer = lex::Status::new(input_file, &mut ts);

    let mut cont_read = true;

    let mut token_list: Vec<lex::Token> = Vec::new();

    while cont_read {
        let token: lex::Token = lexer.get_token().unwrap();

        if token.token_type == lex::TokenTypes::Eof{
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
