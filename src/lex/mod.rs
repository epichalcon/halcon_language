use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use crate::{TS::TableAdmin, parser::Pos};

use crate::parser::Token;

pub struct Status{
    source: String,
    acum: String,
    line_counter: i32,
    char_index: usize,
    pub ts: TableAdmin,
}

impl  Status {
    pub fn new(s: String, table: TableAdmin) -> Self{
        Status { source: s, acum: "".to_string(), line_counter: 1, char_index: 0, ts: table }
    }

    fn get_caracter(&mut self) -> char {
        self.source.chars().nth(self.char_index).unwrap_or('$')
    }

    pub fn get_token(&mut self) -> Option<Token>{ // axioma del automata
        let character: char = Status::get_caracter(self);
        println!("character: {}", character);

        if character.is_alphabetic() {
                self.acum = character.to_string();
                self.char_index += 1;
                return Status::word_state(self);
            }
        else if character.is_numeric() {
                self.char_index += 1;
                return self.num_state();
            }
        else{
            match character {
                '+' => {
                    self.char_index += 1;
                    return self.sum_state();
                }
                '-' => {
                    self.char_index += 1;
                    return self.minus_state();
                }
                '*' => {
                    self.char_index += 1;
                    return self.mul_state();
                }
                '/' => {
                    self.char_index += 1;
                    return self.div_state();
                }
                '%' => {
                    self.char_index += 1;
                    return self.mod_state();
                }
                '=' => {
                    self.char_index += 1;
                    return self.eq_state();
                }
                '!' => {
                    self.char_index += 1;
                    return self.not_state();
                }
                '<' => {
                    self.char_index += 1;
                    return self.less_state();
                }
                '>' => {
                    self.char_index += 1;
                    return self.gr_state();
                }
                ',' => {
                    self.char_index += 1;
                    return Some(Token::Coma);
                }
                ';' => {
                    self.char_index += 1;
                    return Some(Token::Semicolon);
                }
                ':' => {
                    self.char_index += 1;
                    return Some(Token::Colon);
                }
                '(' => {
                    self.char_index += 1;
                    return Some(Token::Opar)
                }
                ')' => {
                    self.char_index += 1;
                    return Some(Token::Cpar)
                }
                '[' => {
                    self.char_index += 1;
                    return Some(Token::Obraq)
                }
                ']' => {
                    self.char_index += 1;
                    return Some(Token::Cbrac)
                }
                '{' => {
                    self.char_index += 1;
                    return Some(Token::Okey)
                }
                '}' => {
                    self.char_index += 1;
                    return Some(Token::Ckey)
                }
                ' ' => {
                    self.char_index += 1;
                    return self.get_token()
                }
                '\t' => {
                    self.char_index += 1;
                    return self.get_token()
                }
                '\n' => {
                    self.char_index += 1;
                    self.line_counter += 1;
                    return self.get_token()
                }
                '$' => {
                    return Some(Token::Eof)
                }
                _ => {
                    println!("rest");
                    None
                }
            }
        }
    }

    fn word_state(&mut self) -> Option<Token>{
        let character: char = Status::get_caracter(self);
        if character.is_alphanumeric() || character == '_'{ 
            self.char_index += 1;
            self.acum.push(character);
            self.word_state()
        }
        else{
            match Token::from_str(&first_letter_to_upper(&self.acum).as_str()){
                Ok(token) =>{
                    Some(token)
                }
                Err(_) => {
                    if self.acum == "true"{
                        Some(Token::ConstBool(true))
                    }
                    else if self.acum == "false"{
                        Some(Token::ConstBool(true))
                    }
                    else{
                        //let pos = self.ts.handle_symbol(self.acum.to_string()).unwrap();
                        // TODO: mete el id en la TS 
                        let pos = 1;
                        Some(Token::Id( Pos {table: 0, position: pos}))
                    }
                }
            }
        }
    }

    fn num_state(&mut self) -> Option<Token>{
        let character: char = Status::get_caracter(self);

        if character.is_numeric(){ 
            self.char_index += 1;
            self.acum.push(character);
            self.num_state()
        }
        else if character == '.'{
            self.char_index += 1;
            self.acum.push(character);
            self.float_inter_state()
        }
        else{
            Some(Token::ConstInt(self.acum.parse::<i32>().unwrap()))
        }
    }

    fn float_inter_state(&mut self) -> Option<Token>{
        let character: char = Status::get_caracter(self);

        if character.is_numeric(){
            self.char_index += 1;
            self.acum.push(character);
            self.float_state()
        }
        else {
            None
        }
    }

    fn float_state(&mut self) -> Option<Token>{
        let character: char = Status::get_caracter(self);

        if character.is_numeric(){ 
            self.char_index += 1;
            self.acum.push(character);
            self.float_state()
        }
        else{
            Some(Token::ConstFloat(self.acum.parse::<f32>().unwrap()))
        }
    }

    fn sum_state(&mut self) -> Option<Token>{
        let character: char = Status::get_caracter(self);

        match character{
            '+' => {
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::Inc)
            }
            '=' =>{
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::SumAsig)
            }
            _ => {
                Some(Token::Plus)
            }
        }
    }

    fn minus_state(&mut self) -> Option<Token>{
        let character: char = Status::get_caracter(self);

        match character {
            '-' => {
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::Dec)
            }
            '=' => {
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::MinAsig)
            }
            '>' => {
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::Arrow)
            }
            _ => {
                Some(Token::Minus)
            }
        }
    }

    fn mul_state(&mut self) -> Option<Token> {
        let character: char = Status::get_caracter(self);

        match character{
            '=' =>{
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::MulAsig)
            }
            _ => {
                Some(Token::Mult)
            }
        }

    }

    fn div_state(&mut self) -> Option<Token> {
        let character: char = Status::get_caracter(self);

        match character{
            '=' =>{
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::DivAsig)
            }
            _ => {
                Some(Token::Div)
            }
        }
    }

    fn mod_state(&mut self) -> Option<Token> {
        Some(Token::Mod)
    }

    fn eq_state(&mut self) -> Option<Token> {
        let character: char = Status::get_caracter(self);

        match character{
            '=' =>{
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::Eq)
            }
            _ => {
                Some(Token::Assig)
            }
        }
    }

    fn not_state(&mut self) -> Option<Token> {
        let character: char = Status::get_caracter(self);

        match character{
            '=' =>{
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::Neq)
            }
            _ => {
                None
            }
        }
    }

    fn less_state(&mut self) -> Option<Token> {
        let character: char = Status::get_caracter(self);

        match character{
            '=' =>{
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::Le)
            }
            _ => {
                Some(Token::Lt)
            }
        }
    }

    fn gr_state(&mut self) -> Option<Token> {
        let character: char = Status::get_caracter(self);

        match character{
            '=' =>{
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::Ge)
            }
            _ => {
                Some(Token::Gt)
            }
        }
    }
}

fn first_letter_to_upper(str: &String) -> String{
    let mut v: Vec<char> = str.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    v.into_iter().collect()
}