mod TS;
mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

use std::fs::{self, File};
use std::sync::mpsc::sync_channel;
use std::{io, thread};

use docopt::Docopt;
use serde::Deserialize;

// use crate::parser::parse;
// use crate::TS::TableAdmin;

static USAGE: &'static str = "
Usage: halcon <input> <output> [-c] [-o] [-b] ...

Options:
    -c              : output c file
    -o              : output object file
    -b              : output binary file
";

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Args {
    arg_input: String,
    arg_output: String,
    flag_c: bool,
    flag_o: bool,
    flag_b: bool,
}

pub fn get_args() -> Args {
    Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit())
}

fn main() {
    println!("Welcome to halcon");
    repl::start(io::stdin(), io::stdout());
}

#[allow(unused_variables)]
fn main1() -> Result<(), i8> {
    let args: Args = get_args();

    let input_file_name = args.arg_input;
    let output_file_name = args.arg_output;

    // se comprueba la validez de los flags

    // let file_result = fs::read_to_string(&input_file_name);
    // let input_file = match file_result {
    //     Ok(file) => file,
    //     Err(_) => {
    //         println!(
    //             "
    //             Not able to open {}",
    //             &input_file_name
    //         );
    //         return Err(3);
    //     }
    // };
    //
    // let file_result = File::create(&output_file_name);
    // let output_file = match file_result {
    //     Ok(file) => file,
    //     Err(_) => {
    //         println!(
    //             "
    //             Not able to open {}",
    //             &output_file_name
    //         );
    //         return Err(3);
    //     }
    // };

    // let (send_to_ts, ts_reciever) = sync_channel(16);
    // let (ts_sender, reciev_from_ts) = sync_channel(16);

    // let mut ts = TableAdmin::new(ts_reciever, ts_sender);
    // let handle = thread::spawn(move || ts.external_interface());
    //
    // let send2 = send_to_ts.clone();
    //
    // let lexer = lex::Status::new(input_file, send_to_ts, reciev_from_ts);
    //
    // match parse(lexer, send2) {
    //     Ok(()) => println!("parse succesfull"),
    //     Err(()) => println!("error in the parsing"),
    // }

    /*
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
    */

    println!("input_file_name: {}", input_file_name);
    println!("output_file_name: {}", output_file_name);
    //println!("token len: {}", token_list.len());

    Ok(())
}
