use crate::{
    token::{Operators, TokenType},
    Lex, Lexer,
};

#[test]
fn test_lexer_input() {
    let inp = "{ ; == >= <= > < != !10\"Hello, World!\"Hello :";

    let mut lexer = Lexer::new(inp);
    let tokens = lexer.tokenize();

    assert_eq!(tokens[0].token_t, TokenType::LCBracket);
    assert_eq!(tokens[1].token_t, TokenType::Semicolon);
    assert_eq!(tokens[2].token_t, TokenType::Op(Operators::EqEq));
    assert_eq!(tokens[3].token_t, TokenType::Op(Operators::MoreEq));
    assert_eq!(tokens[4].token_t, TokenType::Op(Operators::LessEq));
    assert_eq!(tokens[5].token_t, TokenType::Op(Operators::More));
    assert_eq!(tokens[6].token_t, TokenType::Op(Operators::Less));
    assert_eq!(tokens[7].token_t, TokenType::Op(Operators::NEq));
    assert_eq!(tokens[8].token_t, TokenType::Op(Operators::Bang));
    assert!(tokens[9].token_t ==  TokenType::Number && tokens[9].token_v == "10");
    assert!(tokens[10].token_t ==  TokenType::String &&  tokens[10].token_v == "Hello, World!");
    assert!(tokens[11].token_t ==  TokenType::Ident &&  tokens[11].token_v == "Hello");
    assert!(tokens[12].token_t ==  TokenType::Colon &&  tokens[12].token_v == ":");
}
