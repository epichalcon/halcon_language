

enum SymType{}

struct Symbol{
    name: String,
    identifier: i32,
    symbol_type: SymType,
    param_type: SymType,
    ret_type: SymType
}

pub struct Table{
    identifier: i32,
    disp: i32,
    symbol_list: Vec<Symbol>,
}