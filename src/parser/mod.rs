use pomelo;

struct Pos {
    table: i32,
    position: i32,
}

// I won't implement the beguin and the raw statements yet

pomelo!{
    %include {use TS::TableAdmin;}
    %token pub enum Token<'e> {};
    %extra_argument &'e TableAdmin;
    
    %start_symbol program;
    

    
    %type Int i32;
    %type Str String;
    %type Bool bool;
    %type Arr Vec<T>;

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
    
    
    program ::= simple_statement program | complex_statement program | function program | declaration program | beguin_state;
    
    beguin_state ::= Begin Okey inside? Ckey ;

    simple_statement ::= initialization Semicolon | Id Opar pass_param Cpar Semicolon | Print expresion Semicolon | Input Id Semicolon | Return expresion?;
    complex_statement ::= while_state | loop_state | for_state | if_else_state;
    declaration ::= Let Id type_state Semicolon;
    function ::= Fun Id Opar decl_param? Cpar return_type? Okey inside? Ckey;
    
    //function
    decl_param ::= Id Colon type_state another_param?;
    another_param ::= Coma decl_param;
    return_type ::= Arrow type_state;

    // complex
    while_state ::= While Opar expresion Cpar Okey inside? Ckey;

    if_else_state ::= If Opar expresion Cpar Okey inside? Ckey elif_state? else_state?;
    elif_state ::= Elif Opar expresion Cpar Okey inside? Ckey elif_state?
    else_state ::= Else Okey inside? Ckey;
    
    for_state ::= For Opar declaration Semicolon initialization Semicolon inc_dec_state Cpar Okey inside? Ckey;
    inc_dec_state ::= Inc Id | Dec Id | Id Inc | Id Dec;

    loop_state ::= Loop Okey inside? Ckey;

    inside ::= simple_statement inside? | complex_statement inside? | declaration inside?;

    // simple
    initialization ::= Id Assig expresion;

    pass_param ::= Id pass_another_param?;
    pass_another_param ::= Coma pass_param;
    
    type_state ::= Int | Str | Bool | Float | Arr Obraq ConstInt Cbrac Colon type_state;

    expresion ::= expresion logic_operator relation_expression | relation_expression;
    relation_expression ::= relation_expression relation_operator order_expresion | order_expresion; 
    order_expresion ::= order_expression order_operator sum_expresion | sum_expresion; 
    sum_expresion ::= sum_expression sum_operator mul_expresion | mul_expresion; 
    mul_expresion ::= mul_expression mul_operator unary_expresion | unary_expresion; 
    unary_expresion ::= unary_expression unary_operator leaf | leaf; 
    
    leaf ::= Id | Opar expresion Cpar | Id Opar pass_param Cpar | ConstInt | ConstFloat | ConstBool | ConstStr | Id Obraq expresion Cbrac;



    
}