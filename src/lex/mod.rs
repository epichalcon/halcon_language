use std::str::FromStr;
mod token;
#[allow(non_snake_case)]
mod tokenTypes;


use crate::TS::TableAdmin;

pub use self::tokenTypes::TokenTypes;

pub use self::token::Token;

pub struct Status<'a>{
    source: String,
    acum: String,
    line_counter: i32,
    char_index: usize,
    ts: &'a mut TableAdmin,
}

impl <'a> Status<'a> {
    pub fn new(s: String, table: &'a mut TableAdmin) -> Self{
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
                    return Some(Token::new(TokenTypes::Coma, "".to_string()))
                }
                ';' => {
                    self.char_index += 1;
                    return Some(Token::new(TokenTypes::Semicolon, "".to_string()))
                }
                ':' => {
                    self.char_index += 1;
                    return Some(Token::new(TokenTypes::Colon, "".to_string()))
                }
                '(' => {
                    self.char_index += 1;
                    return Some(Token::new(TokenTypes::Opar, "".to_string()))
                }
                ')' => {
                    self.char_index += 1;
                    return Some(Token::new(TokenTypes::Cpar, "".to_string()))
                }
                '[' => {
                    self.char_index += 1;
                    return Some(Token::new(TokenTypes::Obraq, "".to_string()))
                }
                ']' => {
                    self.char_index += 1;
                    return Some(Token::new(TokenTypes::Cbrac, "".to_string()))
                }
                '{' => {
                    self.char_index += 1;
                    return Some(Token::new(TokenTypes::Okey, "".to_string()))
                }
                '}' => {
                    self.char_index += 1;
                    return Some(Token::new(TokenTypes::Ckey, "".to_string()))
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
                    return Some(Token::new(TokenTypes::Eof, "".to_string()))
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
            match TokenTypes::from_str(&first_letter_to_upper(&self.acum).as_str()){
                Ok(token) =>{
                    Some(Token::new(token, "".to_string()))
                }
                Err(_) => {
                    if self.acum == "true" || self.acum == "false"{
                        Some(Token::new(TokenTypes::ConstBool, self.acum.to_string()))
                    }
                    else{
                        let pos = self.ts.handle_symbol(self.acum.to_string()).unwrap();
                        // TODO: mete el id en la TS 
                        Some(Token::new(TokenTypes::Id, pos.to_string()))
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
            Some(Token::new(TokenTypes::ConstInt, self.acum.to_string()))
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
            Some(Token::new(TokenTypes::ConstInt, self.acum.to_string()))
        }
    }

    fn sum_state(&mut self) -> Option<Token>{
        let character: char = Status::get_caracter(self);

        match character{
            '+' => {
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::new(TokenTypes::Inc, "".to_string()))
            }
            '=' =>{
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::new(TokenTypes::SumAsig, "".to_string()))
            }
            _ => {
                Some(Token::new(TokenTypes::Plus, "".to_string()))
            }
        }
    }

    fn minus_state(&mut self) -> Option<Token>{
        let character: char = Status::get_caracter(self);

        match character {
            '-' => {
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::new(TokenTypes::Dec, "".to_string()))
            }
            '=' => {
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::new(TokenTypes::MinAsig, "".to_string()))
            }
            '>' => {
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::new(TokenTypes::Arrow, "".to_string()))
            }
            _ => {
                Some(Token::new(TokenTypes::Minus, "".to_string()))
            }
        }
    }

    fn mul_state(&mut self) -> Option<Token> {
        let character: char = Status::get_caracter(self);

        match character{
            '=' =>{
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::new(TokenTypes::MulAsig, "".to_string()))
            }
            _ => {
                Some(Token::new(TokenTypes::Mult, "".to_string()))
            }
        }

    }

    fn div_state(&mut self) -> Option<Token> {
        let character: char = Status::get_caracter(self);

        match character{
            '=' =>{
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::new(TokenTypes::DivAsig, "".to_string()))
            }
            _ => {
                Some(Token::new(TokenTypes::Div, "".to_string()))
            }
        }
    }

    fn mod_state(&mut self) -> Option<Token> {
        Some(Token::new(TokenTypes::Mod, "".to_string()))
    }

    fn eq_state(&mut self) -> Option<Token> {
        let character: char = Status::get_caracter(self);

        match character{
            '=' =>{
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::new(TokenTypes::Eq, "".to_string()))
            }
            _ => {
                Some(Token::new(TokenTypes::Assig, "".to_string()))
            }
        }
    }

    fn not_state(&mut self) -> Option<Token> {
        let character: char = Status::get_caracter(self);

        match character{
            '=' =>{
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::new(TokenTypes::Neq, "".to_string()))
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
                Some(Token::new(TokenTypes::Le, "".to_string()))
            }
            _ => {
                Some(Token::new(TokenTypes::Lt, "".to_string()))
            }
        }
    }

    fn gr_state(&mut self) -> Option<Token> {
        let character: char = Status::get_caracter(self);

        match character{
            '=' =>{
                self.char_index += 1;
                self.acum.push(character);
                Some(Token::new(TokenTypes::Ge, "".to_string()))
            }
            _ => {
                Some(Token::new(TokenTypes::Gt, "".to_string()))
            }
        }
    }
}

fn first_letter_to_upper(str: &String) -> String{
    let mut v: Vec<char> = str.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    v.into_iter().collect()
}