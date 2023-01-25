use crate::TS::TsOption;

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
    .unwrap();
    
    assert_eq!(args.arg_input, "input.txt".to_string());
    assert_eq!(args.arg_output, "output.txt".to_string());
    assert_eq!(args.flag_c, true);
    assert_eq!(args.flag_b, true);
    assert_eq!(args.flag_o, false);
}

#[test]
#[should_panic]
fn get_args_test5(){
    
    let argv = vec!["halcon", "input.txt", "output.txt", "-d"];
    
    let _: Args = Docopt::new(USAGE).and_then(|d| d.argv(argv.into_iter()).deserialize())
    .unwrap();
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
    let (send_to_ts, ts_reciever) = sync_channel(16);
    let (ts_sender, reciev_from_ts) = sync_channel(16);
    
    let mut ts = TableAdmin::new(ts_reciever, ts_sender);
    
    let handle = thread::spawn(move || ts.external_interface());
    
    
    send_to_ts.send(TsOption::Handle("num".to_string()));

    let err_table = reciev_from_ts.recv().unwrap();
    assert_eq!(err_table, Err("No tables created".to_string()));
    
    send_to_ts.send(TsOption::Create);
    

    send_to_ts.send(TsOption::StartDeclaring);

    send_to_ts.send(TsOption::Handle("num".to_string()));
    let pos_num = reciev_from_ts.recv().unwrap();
    assert_eq!(pos_num, Ok(0));


    send_to_ts.send(TsOption::Handle("num".to_string()));
    let err_dup = reciev_from_ts.recv().unwrap();
    assert_eq!(err_dup, Err("can't declare a variable twice".to_string()));

    send_to_ts.send(TsOption::StopDeclaring);

    send_to_ts.send(TsOption::Handle("a".to_string()));
    let err_not_decl = reciev_from_ts.recv().unwrap();
    assert_eq!(err_not_decl, Err("can't find the variable".to_string()));
    
    send_to_ts.send(TsOption::Handle("num".to_string()));
    let pos_num_2 = reciev_from_ts.recv().unwrap();
    assert_eq!(pos_num, pos_num_2);
}


#[test]
fn handle_symbol_local_test() {
    let (send_to_ts, ts_reciever) = sync_channel(16);
    let (ts_sender, reciev_from_ts) = sync_channel(16);
    
    let mut ts = TableAdmin::new(ts_reciever, ts_sender);
    
    let handle = thread::spawn(move || ts.external_interface());
    send_to_ts.send(TsOption::Create);
    

    send_to_ts.send(TsOption::StartDeclaring);

    send_to_ts.send(TsOption::Handle("num_global".to_string()));
    let pos_num = reciev_from_ts.recv().unwrap();
    assert_eq!(pos_num, Ok(0));

    send_to_ts.send(TsOption::Create);

    send_to_ts.send(TsOption::Handle("num_local".to_string()));
    let pos_num = reciev_from_ts.recv().unwrap();
    assert_eq!(pos_num, Ok(0));
    
    send_to_ts.send(TsOption::StopDeclaring);
    send_to_ts.send(TsOption::Handle("num_global".to_string()));
    let pos_num = reciev_from_ts.recv().unwrap();
    assert_eq!(pos_num, Ok(0));
    
    
    send_to_ts.send(TsOption::StartDeclaring);
    send_to_ts.send(TsOption::Handle("num_global".to_string()));
    let pos_num = reciev_from_ts.recv().unwrap();
    assert_eq!(pos_num, Ok(1));
}