use crate::token::Token;

trait Letter {
    fn is_letter(self) -> bool;
}

impl Letter for u8 {
    fn is_letter(self) -> bool {
        self.is_ascii_alphabetic() || self == b'_'
    }
}
pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token: Token = match self.ch {
            b'+' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::SumAsig
                } else if self.peek_char() == b'+' {
                    self.read_char();
                    Token::Inc
                } else {
                    Token::Plus
                }
            }
            b'-' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::MinAsig
                } else if self.peek_char() == b'>' {
                    self.read_char();
                    Token::Arrow
                } else if self.peek_char() == b'-' {
                    self.read_char();
                    Token::Dec
                } else {
                    Token::Minus
                }
            }
            b'*' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::MulAsig
                } else {
                    Token::Mult
                }
            }
            b'/' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::DivAsig
                } else {
                    Token::Div
                }
            }
            b'%' => Token::Mod,

            b'<' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Le
                } else {
                    Token::Lt
                }
            }
            b'>' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Ge
                } else {
                    Token::Gt
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Neq
                } else {
                    Token::Invalid("!".to_string())
                }
            }
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assig
                }
            }

            b'"' => Token::ConstStr(self.read_string()),
            b',' => Token::Coma,
            b';' => Token::Semicolon,
            b':' => Token::Colon,
            b'(' => Token::Opar,
            b')' => Token::Cpar,
            b'{' => Token::Okey,
            b'}' => Token::Ckey,
            b'[' => Token::Obrac,
            b']' => Token::Cbrac,
            0 => Token::Eof,
            ch => {
                if ch.is_letter() {
                    let id = self.read_identifier();
                    return match id.as_str() {
                        "fun" => Token::Fun,
                        "let" => Token::Let,
                        "if" => Token::If,
                        "elif" => Token::Elif,
                        "else" => Token::Else,
                        "return" => Token::Return,
                        "true" => Token::ConstBool(true),
                        "false" => Token::ConstBool(false),
                        "int" => Token::Int,
                        "str" => Token::Str,
                        "bool" => Token::Bool,
                        "arr" => Token::Arr,
                        "and" => Token::And,
                        "or" => Token::Or,
                        "not" => Token::Not,
                        _ => Token::Id(id),
                    };
                } else if ch.is_ascii_digit() {
                    return Token::ConstInt(self.read_number());
                } else {
                    Token::Invalid((ch as char).to_string())
                }
            }
        };

        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while self.ch.is_letter() {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }

    fn read_number(&mut self) -> i128 {
        let position = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[position..self.position])
            .to_string()
            .parse::<i128>()
            .expect("Not a  valid number, numbers must be i128")
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char()
        }
    }

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    fn read_string(&mut self) -> String {
        self.read_char();
        let position = self.position;

        while self.read_position < self.input.len() && self.ch != b'"' {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+()[]{},;";

        let expected = vec![
            Token::Assig,
            Token::Plus,
            Token::Opar,
            Token::Cpar,
            Token::Obrac,
            Token::Cbrac,
            Token::Okey,
            Token::Ckey,
            Token::Coma,
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input.to_string());

        for (i, token) in expected.iter().enumerate() {
            let new_token = lexer.next_token();
            println!("Test {i} expected: {token}, got: {new_token}");
            assert_eq!(*token, new_token);
        }
    }

    #[test]
    fn test_source_code_first_subset() {
        let input = r#"let five: int = 5;
        let ten: int = 10;
        let hello: str = "hello world";
        let array: arr = [1, 2];
        let my_bool: bool = true;
        
        let add = fun(x,y) -> int {
            x + y;
        }

        let result = add(five, ten);"#;

        let expected = vec![
            Token::Let,
            Token::Id("five".to_string()),
            Token::Colon,
            Token::Int,
            Token::Assig,
            Token::ConstInt(5),
            Token::Semicolon,
            Token::Let,
            Token::Id("ten".to_string()),
            Token::Colon,
            Token::Int,
            Token::Assig,
            Token::ConstInt(10),
            Token::Semicolon,
            Token::Let,
            Token::Id("hello".to_string()),
            Token::Colon,
            Token::Str,
            Token::Assig,
            Token::ConstStr("hello world".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Id("array".to_string()),
            Token::Colon,
            Token::Arr,
            Token::Assig,
            Token::Obrac,
            Token::ConstInt(1),
            Token::Coma,
            Token::ConstInt(2),
            Token::Cbrac,
            Token::Semicolon,
            Token::Let,
            Token::Id("my_bool".to_string()),
            Token::Colon,
            Token::Bool,
            Token::Assig,
            Token::ConstBool(true),
            Token::Semicolon,
            Token::Let,
            Token::Id("add".to_string()),
            Token::Assig,
            Token::Fun,
            Token::Opar,
            Token::Id("x".to_string()),
            Token::Coma,
            Token::Id("y".to_string()),
            Token::Cpar,
            Token::Arrow,
            Token::Int,
            Token::Okey,
            Token::Id("x".to_string()),
            Token::Plus,
            Token::Id("y".to_string()),
            Token::Semicolon,
            Token::Ckey,
            Token::Let,
            Token::Id("result".to_string()),
            Token::Assig,
            Token::Id("add".to_string()),
            Token::Opar,
            Token::Id("five".to_string()),
            Token::Coma,
            Token::Id("ten".to_string()),
            Token::Cpar,
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input.to_string());

        for (i, token) in expected.iter().enumerate() {
            let new_token = lexer.next_token();
            println!("Test {i} expected: {token}, got: {new_token}");
            assert_eq!(*token, new_token);
        }
    }

    #[test]
    fn test_one_char_operands() {
        let input = "-/*5:;
5 < 10 > 5;
";

        let expected = vec![
            Token::Minus,
            Token::Div,
            Token::Mult,
            Token::ConstInt(5),
            Token::Colon,
            Token::Semicolon,
            Token::ConstInt(5),
            Token::Lt,
            Token::ConstInt(10),
            Token::Gt,
            Token::ConstInt(5),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input.to_string());

        for (i, token) in expected.iter().enumerate() {
            let new_token = lexer.next_token();
            println!("Test {i} expected: {token}, got: {new_token}");
            assert_eq!(*token, new_token);
        }
    }

    #[test]
    fn test_if_else_bools() {
        let input = "
if (5 < 10) {
    return true;
} elif {
    return true;
} else {
    return false;
}

and or not
";

        let expected = vec![
            Token::If,
            Token::Opar,
            Token::ConstInt(5),
            Token::Lt,
            Token::ConstInt(10),
            Token::Cpar,
            Token::Okey,
            Token::Return,
            Token::ConstBool(true),
            Token::Semicolon,
            Token::Ckey,
            Token::Elif,
            Token::Okey,
            Token::Return,
            Token::ConstBool(true),
            Token::Semicolon,
            Token::Ckey,
            Token::Else,
            Token::Okey,
            Token::Return,
            Token::ConstBool(false),
            Token::Semicolon,
            Token::Ckey,
            Token::And,
            Token::Or,
            Token::Not,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input.to_string());

        for (i, token) in expected.iter().enumerate() {
            let new_token = lexer.next_token();
            println!("Test {i} expected: {token}, got: {new_token}");
            assert_eq!(*token, new_token);
        }
    }

    #[test]
    fn test_two_char_operands() {
        let input = "
10 == 10;
10 != 9;
<= >=
-- ++
";

        let expected = vec![
            Token::ConstInt(10),
            Token::Eq,
            Token::ConstInt(10),
            Token::Semicolon,
            Token::ConstInt(10),
            Token::Neq,
            Token::ConstInt(9),
            Token::Semicolon,
            Token::Le,
            Token::Ge,
            Token::Dec,
            Token::Inc,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input.to_string());

        for (i, token) in expected.iter().enumerate() {
            let new_token = lexer.next_token();
            println!("Test {i} expected: {token}, got: {new_token}");
            assert_eq!(*token, new_token);
        }
    }
}
