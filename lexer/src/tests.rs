use crate::{
    token::{Operators, TokenType},
    Lex, Lexer,
};

#[test]
fn test_lexer_input() {
    let inp = "{ ; == >= <= > < != !";

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
}
