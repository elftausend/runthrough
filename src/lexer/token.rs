use regex::Regex;

use super::TOKENS;

pub enum TokenKind {
    LeftParan,
    RightParan,
    Sin,
    E,
    Pow,
    Add,
    Sub,
    Mul,
    Div,
    Sqrt,
    Number,
    Argument,
}

pub struct TokenCapture<'a> {
    start: usize,
    end: usize,
    value: &'a str,
}

pub struct Token {
    kind: TokenKind,
    regex: Regex
}

impl Token {
    pub fn new(kind: TokenKind, re: &str) -> Token {
        Token {
            kind,
            regex: Regex::new(re).unwrap()
        }
    }
    pub fn caputure_in_fn<'a>(&self, fn_str: &'a str, token_captures: &mut Vec<TokenCapture<'a>>) {
        for capture in self.regex.captures_iter(fn_str) {
            if let Some(capture) = capture.get(0) {
                token_captures.push(
                    TokenCapture {
                        start: capture.start(),
                        end: capture.end(),
                        value: capture.as_str()
                    }
                )
            }
            
        }
    }
}

pub fn find_tokens(fn_str: &str) {
    let mut token_captures = Vec::<TokenCapture>::new();

    for token in TOKENS.iter() {
        token.caputure_in_fn(fn_str, &mut token_captures)
    }
}