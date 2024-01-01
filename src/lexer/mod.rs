use crate::token::Token;
#[cfg(test)]
mod test;

trait Letter {
    fn is_letter(self) -> bool;
}

impl Letter for u8 {
    fn is_letter(self) -> bool {
        self.is_ascii_alphabetic() || self == b'_'
    }
}

/// The lexer struct is the responsable from taking a string and dividing it into tokens
pub struct Lexer {
    /// The inputed string is transformed into a `Vec<u8>` and is iterated through
    input: Vec<u8>,
    /// The actual position of the pointer in the input
    position: usize,
    /// The next position of the pointer in the input used to observe the next character without
    /// moving the pointer
    read_position: usize,
    /// The actual character
    ch: u8,
}

impl Lexer {
    /**
    Returns a Lexer with the input with the `String` given

    # Arguments

    * `input` - the `String` that will be tokenized

    # Examples 
    ```
    let lex = Lexer::new(contents);

    ```
    */
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


    /**
    The character pointed to by `self.position` is loaded in `self.ch` and both pointers are moved forward
    If the pointer is out of bounds the null character is loaded

    # Arguments

    no arguments

    */
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    /**
    Returns the next `Token` detected from the input moving the pointer to the character in front of the last token read

    # Arguments

    no arguments

    # Examples 
    ```
    let lexer = Lexer::new(contents);

    let new_token = lexer.next_token();
    ```
    */
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


    /**
    When a word is detected the function will read the word and return a `String` with the content

    # Arguments

    no arguments
    ```
    if ch.is_letter() {
        let id = self.read_identifier();
    }
    ```
    */
    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while self.ch.is_letter() {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }


    /**
    When a digit is detected the function will read the number and return a `i128` with the content

    # Arguments

    no arguments
    ```
    if ch.is_digit() {
        let id = self.read_number();
    }
    ```
    */
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


    /**
    When a string is detected the function will read the string and return a `String` with the content

    # Arguments

    no arguments
    ```
    if ch == b'"' {
        let string = self.read_string();
    }
    ```
    */

    fn read_string(&mut self) -> String {
        self.read_char();
        let position = self.position;

        while self.read_position < self.input.len() && self.ch != b'"' {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }

    /**
    The pointer will move to the next non white space character

    # Arguments

    no arguments
    */
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char()
        }
    }

    /**
    Returns the character pointed by `self.read_position` in u8 form 
    If the 'self.read_position' is out of bounds, the function returns the null character

    # Arguments

    no arguments
    ```
    if self.peek_char() == b'=' {
        self.read_char();
    } 
    ```
    */
    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

}

