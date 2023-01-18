

#[derive(Clone)]
enum SymType{
    None, 
}

#[allow(dead_code)]
#[derive(Clone)]
struct Symbol{
    name: String,
    identifier: i32,
    symbol_type: SymType,
    param_type: SymType,
    ret_type: SymType
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Table{
    identifier: i32,
    pub disp: i32,
    symbol_list: Vec<Symbol>,
}

#[allow(dead_code)]
impl Table {
    pub fn new(id: i32) -> Self{
        Table { identifier: id, disp: 0, symbol_list: vec![]}
    }

    pub fn search_for_name(&self, name: String) -> i32{
        let mut i:i32 = 0;
        for sym in &self.symbol_list{
            if sym.name == name{
                return i;
            }
            i += 1;
        }
        return -1;
    }

    pub fn add_symbol(&mut self, s_name: String) -> i32{
        let id: i32 = self.symbol_list.len().try_into().unwrap();

        self.symbol_list.push(
            Symbol{
                name: s_name,
                identifier: id,
                symbol_type: SymType::None,
                param_type: SymType::None,
                ret_type: SymType::None
            }
        );
        id
    }
}