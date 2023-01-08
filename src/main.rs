use std::fs::File;
use std::str::FromStr;

use strum_macros::EnumString;

use std::env;

#[derive(EnumString)]
enum Flags {
    o, c, b
}


fn main() -> Result<(), i8> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 { // solo se llama al programa. Se muestra el help
        println!("
            HELP:

            Mode of use:
            `halcon <source file name> <output file name> [flags]`

            Flags:
                -c              : output c file
                -o              : output object file
                -b              : output binary file
            ");
        return Ok(());
    }
    else if args.len() < 3 { // se no se especifica la entrada o la salida 
        println!("
                Incorrect usage!

                correct usage:
                `halcon <source file name> <output file name> [flags]`
            ");
        return Err(1);

    }
    

    let input_file_name = &args[1];
    let output_file_name = &args[2];
    let mut input_flags: Vec<String> = (&args[3..]).to_vec();


    if input_file_name.starts_with('-') || output_file_name.starts_with('-'){// no se ha pasado o la entrada o la salida
        println!("
                Incorrect usage!

                correct usage:
                `halcon <source file name> <output file name> [flags]`
            ");
        return Err(1);
    }


    // se han pasado mas argumentos de lo que se debian
    let prev_length = input_flags.len();
    input_flags.retain(|flag| flag.starts_with('-'));
    if prev_length != input_flags.len() {
        println!("
                Too many arguments!

                Write ´halcon´ to get help
            ");
        return Err(2);

    }


    // la lista de flags validos
    let mut output_file_types: Vec<Flags> = Vec::new();

    // se comprueba la validez de los flags
    for flag in input_flags{
        let enum_flag: Flags = Flags::from_str(flag.replace("-", "").as_str())
            .expect("       
            invalid flag {flag}
            valid flags are:
                -c              : output c file
                -o              : output object file
                -b              : output binary file
            ");
        match enum_flag{
            Flags::o => { output_file_types.push(enum_flag); continue; }
            Flags::b => { output_file_types.push(enum_flag); continue; }
            Flags::c => { output_file_types.push(enum_flag); continue; }
        }
    }

    let mut file_result = File::open(input_file_name);
    let input_file = match file_result {
        Ok(file) => file ,
        Err(_) => {
            println!("
                Not able to open {}", input_file_name);
            return Err(3)
        }
    };

    file_result = File::create(output_file_name);
    let output_file = match file_result {
        Ok(file) => file ,
        Err(_) => {
            println!("
                Not able to open {}", output_file_name);
            return Err(3)
        }
    };

    

    println!("input_file_name: {}", input_file_name);
    println!("output_file_name: {}", output_file_name);
    println!("flags len: {}", output_file_types.len());


    Ok(())
}
