use std::sync::mpsc::SyncSender;

use pomelo::pomelo;
use crate::{lex::Status, TS::TsOption};

pub use self::parser::{Parser, Token};

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Pos {
    pub(crate) table: i32,
    pub(crate) position: i32,
}

// I won't implement the beguin and the raw statements yet

pomelo!{
    %include {use crate::TS::TsOption;
            use crate::parser::Pos;
            use strum_macros::EnumString;}
    %token #[derive(Debug, EnumString, PartialEq, Clone)]
            pub enum Token{};
    %parser pub struct Parser{};
    %extra_argument std::sync::mpsc::SyncSender<TsOption>;

    %start_symbol program;



    %type Int;
    %type Str;
    %type Bool;
    %type Float;
    %type Arr;

    %type Plus; 
    %type Minus;
    %type Mult;
    %type Div;
    %type Mod;

    %type Eq;
    %type Neq;
    %type Lt;
    %type Gt;
    %type Le;
    %type Ge;

    %type And;
    %type Or;
    %type Not;

    %type Inc;
    %type Dec;

    %type Assig;
    %type SumAsig;
    %type MinAsig;
    %type MulAsig;
    %type DivAsig;

    %type Id Pos;
     
    %type Fun;
    %type Let;
    %type Arrow; 
    %type Return;
    %type Raw;
    %type Begin;
    %type If;
    %type Elif;
    %type Else;
    %type For;
    %type Loop;
    %type While;

    %type Input;
    %type Print;
     
    %type Coma;
    %type Semicolon;
    %type Colon;
     
    %type Opar;
    %type Cpar;
    %type Obraq;
    %type Cbrac;
    %type Okey;
    %type Ckey;

    %type ConstInt i32;
    %type ConstFloat f32;
    %type ConstStr String;
    %type ConstBool bool;
    
    %type Eof;
 
    program ::= continue_state;

    continue_state ::= simple_statement continue_state? Eof;
    continue_state ::= complex_statement continue_state? Eof;
    continue_state ::= function continue_state? Eof;
    continue_state ::= declaration continue_state? Eof;
    continue_state ::= begin_state continue_state? Eof;

    begin_state ::= Begin Okey inside? Ckey ;

    simple_statement ::= initialization Semicolon;
    simple_statement ::= Id Opar pass_param Cpar Semicolon;
    simple_statement ::= Print expresion Semicolon;
    simple_statement ::= Input Id Semicolon;
    simple_statement ::= Return expresion? Semicolon;
    simple_statement ::= asignation Semicolon;

    complex_statement ::= while_state;
    complex_statement ::= loop_state;
    complex_statement ::= for_state;
    complex_statement ::= if_else_state;

    declaration ::= Let Id Colon type_state Semicolon;
    function ::= Fun Id Opar decl_param? Cpar return_type? Okey inside? Ckey;

    //function
    decl_param ::= Id Colon type_state another_param?;
    another_param ::= Coma decl_param;
    return_type ::= Arrow type_state;

    //complex
    while_state ::= While Opar expresion Cpar Okey inside? Ckey;

    if_else_state ::= If Opar expresion Cpar Okey inside? Ckey elif_state? else_state?;
    elif_state ::= Elif Opar expresion Cpar Okey inside? Ckey elif_state?;
    else_state ::= Else Okey inside? Ckey;

    for_state ::= For Opar declaration initialization Semicolon relation_expression Semicolon inc_dec_state Cpar Okey inside? Ckey;
    inc_dec_state ::= Id inc_dec_tok;

    inc_dec_tok ::= Inc;
    inc_dec_tok ::= Dec;

    loop_state ::= Loop Okey inside? Ckey;

    inside ::= inside? simple_statement;
    inside ::= inside? complex_statement;
    inside ::= inside? declaration;

    // simple
    initialization ::= Id Assig expresion;
    asignation ::= Id asig_tok expresion;
    
    asig_tok ::= SumAsig;
    asig_tok ::= MinAsig;
    asig_tok ::= MulAsig;
    asig_tok ::= DivAsig;

    pass_param ::= Id pass_another_param?;
    pass_another_param ::= Coma pass_param;

    type_state ::= Int;
    type_state ::= Str;
    type_state ::= Bool;
    type_state ::= Float;
    type_state ::= Arr Obraq ConstInt Cbrac Colon type_state;

    expresion ::= expresion logic_operator relation_expression;
    expresion ::= relation_expression;
    relation_expression ::= relation_expression relation_operator order_expresion; 
    relation_expression ::= order_expresion; 
    order_expresion ::= order_expresion order_operator sum_expresion; 
    order_expresion ::= sum_expresion; 
    sum_expresion ::= sum_expresion sum_operator mul_expresion; 
    sum_expresion ::= mul_expresion; 
    mul_expresion ::= mul_expresion mul_operator unary_expresion; 
    mul_expresion ::= unary_expresion; 
    unary_expresion ::= unary_expresion unary_operator leaf; 
    unary_expresion ::= leaf; 

    logic_operator ::= And; 
    logic_operator ::= Or; 
    relation_operator ::= Eq;
    relation_operator ::= Neq;
    order_operator ::= Ge;
    order_operator ::= Lt;
    order_operator ::= Gt;
    order_operator ::= Le;
    sum_operator ::= Plus;
    sum_operator ::= Minus;
    mul_operator ::= Mult;
    mul_operator ::= Div;
    mul_operator ::= Mod;
    unary_operator ::= Inc;
    unary_operator ::= Dec;
    unary_operator ::= Not;

    leaf ::= Id;
    leaf ::= Opar expresion Cpar;
    leaf ::= Id Opar pass_param Cpar ;
    leaf ::= ConstInt ;
    leaf ::= ConstFloat;
    leaf ::= ConstBool ;
    leaf ::= ConstStr ;
    leaf ::= Id Obraq expresion Cbrac;
}

pub fn parse (mut lex: Status, send_to_ts: SyncSender<TsOption>) -> Result<(), ()> {
    let mut p = Parser::new(send_to_ts);
    let status = Ok(());
    loop {
        let token: Token = match lex.get_token() {
            Some(t) => t.clone(),
            None => break
        };
        let ctok: Token = token.clone();
        let res = p.parse(token);
        match res {
            Ok(()) => {println!("parsed {:?} ", ctok); if ctok == Token::Eof {return status}},
            Err(()) => {println!("failed at {:?} ", ctok); return Err(())},
        }
    }
    status
}
