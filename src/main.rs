mod ast;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;
mod token;

use std::io;

use docopt::Docopt;
use serde::Deserialize;

static USAGE: &'static str = "
Usage: halcon [-s <input>]

    default: runns the language REPL
    Option:
        -s <input>: an input file can be specified from whitch to get the code
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

    println!("input_file_name: {}", input_file_name);
    println!("output_file_name: {}", output_file_name);
    //println!("token len: {}", token_list.len());

    Ok(())
}
