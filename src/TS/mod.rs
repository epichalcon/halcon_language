use std::sync::mpsc::{Receiver, SyncSender};

use self::table::Table;

pub(crate) mod table;

#[allow(dead_code)]
#[derive(Debug)]
pub enum TsOption {
    Create, Destroy, Handle(String), StartDeclaring, StopDeclaring,  End
}


pub struct TableAdmin{
    ts_stack: Vec<table::Table>,
    declaring: bool,
    in_channel: Receiver<TsOption>,
    out_channel: SyncSender<Result<i32, String>>,
}

#[allow(dead_code)]
impl TableAdmin {
    pub fn new(in_cannel: Receiver<TsOption>, out_channel: SyncSender<Result<i32, String>>) -> Self{
        TableAdmin{
            ts_stack: vec![],
            declaring: false,
            in_channel: in_cannel,
            out_channel: out_channel
        }
    }
    
    pub fn external_interface(&mut self) {
        loop {
            match self.in_channel.recv(){
                Ok(option) => {
                    match option{
                        TsOption::Create => self.create_table(),
                        TsOption::Destroy => self.destroy_table(),
                        TsOption::Handle(name) => {let res = self.handle_symbol(name);
                                                    self.out_channel.send(res).unwrap();},
                        TsOption::StartDeclaring => self.declaring = true,
                        TsOption::StopDeclaring => self.declaring = false,
                        TsOption::End => break,
                    }
                },
                Err(_) => {
                    break
                },
            }
        }
    }

    fn create_table(&mut self){
        self.ts_stack.push(Table::new(self.ts_stack.len().try_into().unwrap()))
    }

    fn destroy_table(&mut self){
        self.ts_stack.pop();
    }

    // Called by lex to add a symbol to a table or get the symbol position
    fn handle_symbol(&mut self,name: String) -> Result<i32, String> {
        print!("{}", name);
        
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
