use pomelo::pomelo;

pub struct Pos {
    table: i32,
    position: i32,
}

// I won't implement the beguin and the raw statements yet

pomelo!{
    %include {use crate::TS::TableAdmin;
            use crate::parser::Pos;}
    %token pub enum Token{};
    %parser pub struct Parser<'e>{};
    %extra_argument &'e TableAdmin;

    %start_symbol program;



    %type Int i32;
    %type Str String;
    %type Bool bool;
    %type Float f32;
    %type ArrI Vec<i32>;
    %type ArrS Vec<String>;
    %type ArrB Vec<bool>;
    %type ArrF Vec<f32>;

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

    %type ConstInt;
    %type ConstFloat;
    %type ConstStr;
    %type ConstBool;
 
    program ::= continue_state;

//    continue_state ::=  simple_statement continue_state?;
    continue_state ::= complex_statement continue_state?;
//    continue_state ::= function continue_state?;
//    continue_state ::= declaration continue_state?;
//    continue_state ::= begin_state continue_state?;

//    begin_state ::= Begin Okey inside? Ckey ;

//    simple_statement ::= initialization Semicolon;
//    simple_statement ::= Id Opar pass_param Cpar Semicolon;
//    simple_statement ::= Print expresion Semicolon;
//    simple_statement ::= Input Id Semicolon;
//    simple_statement ::= Return expresion? Semicolon;

//    complex_statement ::= while_state;
//    complex_statement ::= loop_state;
    complex_statement ::= for_state;
//    complex_statement ::= if_else_state;

    declaration ::= Let Id type_state Semicolon;
//    function ::= Fun Id Opar decl_param? Cpar return_type? Okey inside? Ckey;
 
    //function
//    decl_param ::= Id Colon type_state another_param?;
//    another_param ::= Coma decl_param;
//    return_type ::= Arrow type_state;

    // complex
//    while_state ::= While Opar expresion Cpar Okey inside? Ckey;

//    if_else_state ::= If Opar expresion Cpar Okey inside? Ckey elif_state? else_state?;
//    elif_state ::= Elif Opar expresion Cpar Okey inside? Ckey elif_state?;
//    else_state ::= Else Okey inside? Ckey;

    for_state ::= For Opar declaration initialization inc_dec_state  Semicolon Cpar Okey inside? Ckey;
    inc_dec_state ::= Id inc_dec_tok;

    inc_dec_tok ::= Inc|Dec;

//    loop_state ::= Loop Okey inside? Ckey;

//    inside ::= inside? simple_statement;
    inside ::= inside? complex_statement;
    inside ::= inside? declaration;

    // simple
    initialization ::= Id Assig expresion;

    pass_param ::= Id pass_another_param?;
    pass_another_param ::= Coma pass_param;

    type_state ::= Int | Str | Bool | Float | Arr Obraq ConstInt Cbrac Colon type_state;

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

    logic_operator ::= And|Or; 
    relation_operator ::= Eq|Neq;
    order_operator ::= Lt|Gt|Le|Ge;
    sum_operator ::= Plus|Minus;
    mul_operator ::= Mult|Div|Mod;
    unary_operator ::= Inc|Dec|Not;

    leaf ::= Id | Opar expresion Cpar | Id Opar pass_param Cpar | ConstInt | ConstFloat | ConstBool | ConstStr | Id Obraq expresion Cbrac;
}