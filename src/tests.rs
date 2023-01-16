use super::*;

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