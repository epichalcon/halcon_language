
use strum_macros::EnumString;

#[derive(Debug, EnumString, PartialEq)]
pub enum TokenTypes{
    Int, Str, Bool, Arr,
    Plus, Minus, Mult, Div, Mod,
    Eq, Neq, Lt, Gt, Le, Ge,
    And, Or, Not,
    Inc, Dec,
    Assig, SumAsig, MinAsig, MulAsig, DivAsig,
    Id,
    Fun, Let, Arrow, 
    Return,
    Raw, Begin,
    If, Elif, Else, For, Loop, While,
    Input, Print,
    Coma, Semicolon, Colon, Opar, Cpar, Obraq, Cbrac, Okey, Ckey,
    ConstInt, ConstStr, ConstBool, ConstArr,
    Eof,
}