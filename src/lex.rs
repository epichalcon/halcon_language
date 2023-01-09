

#[derive(Debug)]
struct Token {
    tokenType: TokenTypes,
    attribute: String,
}
impl Token{
    fn new( t: TokenTypes, attr: String) -> Self {
        Token {
            tokenType = t;
            attribute = attr;
        }
    }
}

enum TokenTypes{
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
}

struct Status{
    source: String,
    acum: String,
    lineCounter: i32,
    charIndex: i32,
}
impl Status {
    pub fn getToken(&self) -> Token{ // axioma del automata
        let character: char = self.source.chars().nth(self.charIndex).unwrap();

        if character.is_alphabetic() {
                acum = character;
                self.charIndex += 1;
                return self.wordState();
            }
        else if character.is_numeric() {
                self.charIndex += 1;
                return self.numState();
            }
        else if character == '+'{
                self.charIndex += 1;
                return self.sumState();
            }
        else if character == ' ' || character == '\t'{
                self.charIndex += 1;
                return self.getToken()
            }
        else if character == '\n'{
                self.charIndex += 1;
                self.lineCounter += 1;
                return self.getToken()
            }
        return Token::new(TokenTipes::Fun, "");
    }
    fn wordState() -> Token{
        return Token::new(TokenTipes::Fun, "");
    }
}
