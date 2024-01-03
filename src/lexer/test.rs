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
<= >= // hola
-- ++
break
// hola
//";

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
        Token::Break,
        Token::Eof,
    ];

    let mut lexer = Lexer::new(input.to_string());

    for (i, token) in expected.iter().enumerate() {
        let new_token = lexer.next_token();
        println!("Test {i} expected: {token}, got: {new_token}");
        assert_eq!(*token, new_token);
    }
}
