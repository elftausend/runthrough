use regex::Regex;

use super::TOKENS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenCapture<'a> {
    start: usize,
    end: usize,
    value: &'a str,
    kind: TokenKind,
}

#[derive(Debug)]
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
                        value: capture.as_str(),
                        kind: self.kind,
                    }
                )
            }
            
        }
    }
}

pub fn find_tokens(fn_str: &str) -> Vec<TokenCapture> {
    let mut token_captures = Vec::<TokenCapture>::new();

    for token in TOKENS.iter() {
        token.caputure_in_fn(fn_str, &mut token_captures)
    }

    token_captures.sort_by(|a, b| a.start.cmp(&b.start));
    token_captures
}

#[cfg(test)]
mod tests {
    use crate::lexer::{find_tokens, token::TokenCapture, TokenKind};

    #[test]
    fn test_find_tokens() {
        //let input = "y = x^2 + x - sin(x) / e^x";
        let input = "x / (x+1)";
        let tokens = find_tokens(input);
        assert_eq!(tokens, vec![
            TokenCapture { start: 0, end: 1, value: "x", kind: TokenKind::Argument }, 
            TokenCapture { start: 2, end: 3, value: "/", kind: TokenKind::Div }, 
            TokenCapture { start: 4, end: 5, value: "(", kind: TokenKind::LeftParan },
            TokenCapture { start: 5, end: 6, value: "x", kind: TokenKind::Argument }, 
            TokenCapture { start: 6, end: 7, value: "+", kind: TokenKind::Add }, 
            TokenCapture { start: 7, end: 8, value: "1", kind: TokenKind::Number }, 
            TokenCapture { start: 8, end: 9, value: ")", kind: TokenKind::RightParan }
        ]);
    }
    
    #[test]
    fn test_find_tokens2() {
        let input = "sin(x) / x";
        let tokens = find_tokens(input);
        println!("tokens: {tokens:?}");
    }

    #[test]
    fn test_find_tokens3() {
        let input = "x + x * x";
        let tokens = find_tokens(input);
        println!("tokens: {tokens:?}");
    }
}
