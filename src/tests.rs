use super::*;

// main TESTS
#[test]
fn get_args_test1(){
    
    let argv = vec!["halcon", "input.txt", "output.txt"];
    
    let args: Args = Docopt::new(USAGE).and_then(|d| d.argv(argv.into_iter()).deserialize())
    .unwrap_or_else(|e| e.exit());
    
    assert_eq!(args.arg_input, "input.txt".to_string());
    assert_eq!(args.arg_output, "output.txt".to_string());
    assert_eq!(args.flag_c, false);
    assert_eq!(args.flag_b, false);
    assert_eq!(args.flag_o, false);
}

#[test]
fn get_args_test2(){
    
    let argv = vec!["halcon", "input.txt", "output.txt", "-c"];
    
    let args: Args = Docopt::new(USAGE).and_then(|d| d.argv(argv.into_iter()).deserialize())
    .unwrap_or_else(|e| e.exit());
    
    assert_eq!(args.arg_input, "input.txt".to_string());
    assert_eq!(args.arg_output, "output.txt".to_string());
    assert_eq!(args.flag_c, true);
    assert_eq!(args.flag_b, false);
    assert_eq!(args.flag_o, false);

}

#[test]
fn get_args_test3(){
    
    let argv = vec!["halcon", "input.txt", "output.txt", "-c", "-o", "-b"];
    
    let args: Args = Docopt::new(USAGE).and_then(|d| d.argv(argv.into_iter()).deserialize())
    .unwrap_or_else(|e| e.exit());
    
    assert_eq!(args.arg_input, "input.txt".to_string());
    assert_eq!(args.arg_output, "output.txt".to_string());
    assert_eq!(args.flag_c, true);
    assert_eq!(args.flag_b, true);
    assert_eq!(args.flag_o, true);

}

#[test]
fn get_args_test4(){
    
    let argv = vec!["halcon", "input.txt", "output.txt", "-c", "-b"];
    
    let args: Args = Docopt::new(USAGE).and_then(|d| d.argv(argv.into_iter()).deserialize())
    .unwrap_or_else(|e| e.exit());
    
    assert_eq!(args.arg_input, "input.txt".to_string());
    assert_eq!(args.arg_output, "output.txt".to_string());
    assert_eq!(args.flag_c, true);
    assert_eq!(args.flag_b, true);
    assert_eq!(args.flag_o, false);
}


// TS TESTS ****************************************************
#[test]
fn add_symbol_test() {
    let mut ts = TS::table::Table::new(0);

    ts.add_symbol("num".to_string());
    
    let res = ts.search_for_name("num".to_string());
    assert_eq!(res, 0);
}

#[test]
fn search_for_name_test() {
    let mut ts = TS::table::Table::new(0);

    ts.add_symbol("num".to_string());
    ts.add_symbol("a".to_string());
    ts.add_symbol("b".to_string());
    
    let res = ts.search_for_name("num".to_string());
    assert_eq!(res, 0);
    let res = ts.search_for_name("a".to_string());
    assert_eq!(res, 1);
    let res = ts.search_for_name("b".to_string());
    assert_eq!(res, 2);

    let res = ts.search_for_name("c".to_string());
    assert_eq!(res, -1);
}


#[test]
fn handle_symbol_global_test() {
    let mut ts = TS::TableAdmin::new();

    let err_table = ts.handle_symbol("num".to_string());
    assert_eq!(err_table, Err("No tables created".to_string()));
    
    ts.create_table();
    

    ts.declaring = true;
    let pos_num = ts.handle_symbol("num".to_string());
    assert_eq!(pos_num, Ok(0));

    let err_dup = ts.handle_symbol("num".to_string());
    assert_eq!(err_dup, Err("can't declare a variable twice".to_string()));

    ts.declaring = false;
    let err_not_decl = ts.handle_symbol("a".to_string());
    assert_eq!(err_not_decl, Err("can't find the variable".to_string()));
    
    let pos_num_2 = ts.handle_symbol("num".to_string());
    assert_eq!(pos_num, pos_num_2);
}


#[test]
fn handle_symbol_local_test() {
    let mut ts = TS::TableAdmin::new();

    ts.create_table();
    
    ts.declaring = true;
    let pos_num = ts.handle_symbol("num_global".to_string());
    assert_eq!(pos_num, Ok(0));

    ts.create_table();

    let pos_num = ts.handle_symbol("num_local".to_string());
    assert_eq!(pos_num, Ok(0));
    
    ts.declaring = false;
    let pos_num = ts.handle_symbol("num_global".to_string());
    assert_eq!(pos_num, Ok(0));
    
    
    ts.declaring = true;
    let pos_num = ts.handle_symbol("num_global".to_string());
    assert_eq!(pos_num, Ok(1));
}