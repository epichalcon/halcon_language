use self::table::Table;

pub(crate) mod table;

pub struct TableAdmin{
    ts_stack: Vec<table::Table>,
    pub declaring: bool,
}
impl TableAdmin {
    pub fn new() -> Self{
        TableAdmin{
            ts_stack: vec![],
            declaring: false,
        }
    }

    pub fn create_table(&mut self){
        self.ts_stack.push(Table::new(self.ts_stack.len().try_into().unwrap()))
    }

    pub fn destroy_table(&mut self){
        self.ts_stack.pop();
    }

    // Called by lex to add a symbol to a table or get the symbol position
    pub fn handle_symbol(&mut self,name: String) -> Result<i32, String> {
        
        if self.ts_stack.len() == 0 {
            return  Err("No tables created".to_string());
        }
        
        let n: &str = name.as_str();
        
        let res = self.handle_symbol_helper(n.to_string(), self.ts_stack.len() -1);
        
        if res == Err("can't find the variable".to_string()){
            let res = self.handle_symbol_helper(n.to_string(), 0);
            return res;
        }
        res
    }

    fn handle_symbol_helper(&mut self,name: String, table_pos: usize) -> Result<i32, String> {
        //
        // gets the position of the symbol in the top table of the stack
        let pos: i32 = self.ts_stack.get(table_pos)
                                    .unwrap()
                                    .search_for_name(name.to_string());
        
        if pos != -1 && !self.declaring {
            return Ok(pos);
        }
        
        if pos != -1 && self.declaring {
            return Err("can't declare a variable twice".to_string());
        }
        
        if pos == -1 && self.declaring {
            let mut table: Table = self.ts_stack.remove(self.ts_stack.len() -1);
            let pos = table.add_symbol(name.to_string());
            self.ts_stack.push(table);
            return Ok(pos);
        }
        
        Err("can't find the variable".to_string())
    }
}