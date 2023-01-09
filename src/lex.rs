use std::{collections::HashSet, str::FromStr};

use strum_macros::EnumString;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenTypes,
    pub attribute: String,
}
impl Token{
    fn new( t: TokenTypes, attr: String) -> Self {
        Token {
            token_type: t,
            attribute: attr
        }
    }
}

#[derive(Debug, EnumString, PartialEq)]
pub enum TokenTypes{
    Int, Str, Bool, Arr,
    Plus, Minus, Mult, Div, Mod,
    Eq, Neq, Lt, Gt, Le, Ge,
    And, Or, Not,
    Psint, Princ, Psdec, Prdec,
    Assig, SumAsig, MinAsig, MulAsig, DivAsig,
    Id,
    Fun, Let, Arrow, 
    Return,
    Raw, Begin,
    If, Elif, Else, For, Loop, While,
    Coma, Semicolon, Colon, Opar, Cpar, Obraq, Cbrac, Okey, Ckey,
    ConstInt, ConstStr, ConstBool, ConstArr,
    Eof,
}




pub struct Status{
    source: String,
    acum: String,
    line_counter: i32,
    char_index: usize,
}

impl Status {
    pub fn new(s: String) -> Self{
        Status { source: s, acum: "".to_string(), line_counter: 1, char_index: 0 }
    }

    fn getCaracter(&mut self) -> char {
        self.source.chars().nth(self.char_index).unwrap_or('$')
    }

    pub fn get_token(&mut self) -> Option<Token>{ // axioma del automata
        let character: char = Status::getCaracter(self);

        if character.is_alphabetic() {
                self.acum = character.to_string();
                self.char_index += 1;
                return Status::word_state(self);
            }
        else if character.is_numeric() {
                self.char_index += 1;
                return self.num_state();
            }
        else if character == '+'{
                self.char_index += 1;
                return self.sum_state();
            }
        else if character == ' ' || character == '\t'{
                self.char_index += 1;
                return self.get_token()
            }
        else if character == '\n'{
                self.char_index += 1;
                self.line_counter += 1;
                return self.get_token()
            }
            //...
        else{
            return None;
        }
    }

    fn word_state(&mut self) -> Option<Token>{
        let character: char = Status::getCaracter(self);
        if character.is_alphanumeric() || character == '_'{ 
            self.acum.push(character);
            return self.word_state();
        }
        else{
            match TokenTypes::from_str(&first_letter_to_upper(&self.acum).as_str()){
                Ok(token) => return Some(Token::new(token, "-".to_string())),
                Err(_) => return Some(Token::new(TokenTypes::Id, "1".to_string()))
            }
        }
    }

    fn num_state(&mut self) -> Option<Token>{
        let character: char = Status::getCaracter(self);

        if character.is_alphanumeric() || character == '_'{ 
            self.acum.push(character);
            return self.word_state();
        }
        else{
            None
        }
    }

    fn sum_state(&mut self) -> Option<Token>{
        None
    }
}

fn first_letter_to_upper(str: &String) -> String{
    let mut v: Vec<char> = str.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    v.into_iter().collect()
}