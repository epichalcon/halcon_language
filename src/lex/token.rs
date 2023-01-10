use crate::lex::tokenTypes::TokenTypes;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenTypes,
    pub attribute: String,
}
impl Token{
    pub fn new( t: TokenTypes, attr: String) -> Self {
        Token {
            token_type: t,
            attribute: attr
        }
    }
}