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


    else if args.len() < 3 {
        println!("
                Incorrect usage!

                correct usage:
                `halcon <source file name> <output file name> [flags]`
            ");
        return Err(1);

    }
    
    let input_file = &args[1];
    let output_file = &args[2];

    let mut input_flags: Vec<String> = (&args[3..]).to_vec();

    let prev_length = input_flags.len();
    
    input_flags.retain(|flag| flag.starts_with('-'));

    if prev_length != input_flags.len() {
        println!("
                Too many arguments!

                Write ´halcon´ to get help
            ");
        return Err(2);

    }

    let mut output_file_types: Vec<Flags> = Vec::new();

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

    /*
    let section = SectionName::from_str(
        broken_section.as_str()).expect("No sections with that name");
*/
    

    println!("input_file: {}", input_file);
    println!("output_file: {}", output_file);
    println!("flags len: {}", output_file_types.len());


    Ok(())
}
