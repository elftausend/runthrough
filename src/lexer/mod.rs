mod token;
use lazy_static::lazy_static;

pub use crate::lexer::token::{Token, TokenKind, find_tokens};

lazy_static! {
    pub static ref TOKENS: Vec<Token> = vec![
        Token::new(TokenKind::LeftParan, "[(]"),
        Token::new(TokenKind::RightParan, "[)]"),
        Token::new(TokenKind::Sin, "sin"),
        Token::new(TokenKind::E, "e"),
        Token::new(TokenKind::Pow, r"[\^]"),
        Token::new(TokenKind::Add, "[+]"),
        Token::new(TokenKind::Sub, "[-]"),
        Token::new(TokenKind::Mul, "[*]"),
        Token::new(TokenKind::Div, "[/]"),
        Token::new(TokenKind::Sqrt, "sqrt"),
        Token::new(TokenKind::Number, r"\d+"),
        Token::new(TokenKind::Argument, "x"),
    ];
}
