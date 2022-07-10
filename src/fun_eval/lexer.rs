use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl TokenKind {
    pub fn is_op(self) -> bool {
        !matches!(self, Self::Number | Self::Argument | Self::E)
    }
    pub fn is_unary(self) -> bool {
        matches!(self, Self::Sin | Self::Sqrt)
    }
}

/*
impl PartialOrd for TokenKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TokenKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if *self == Self::Add || *self == Self::Sub && *other == Self::Mul || *other == Self::Div {
            return std::cmp::Ordering::Less;
        }
        std::cmp::Ordering::Equal
    }
}*/

lazy_static! {
    pub static ref TOKENS: Vec<Token> = vec![
        Token::new(TokenKind::LeftParan, "[(]"),
        Token::new(TokenKind::RightParan, "[)]"),
        Token::new(TokenKind::Sin, ".sin"),
        Token::new(TokenKind::E, "e"),
        Token::new(TokenKind::Pow, r"pow"),
        Token::new(TokenKind::Add, "[+]"),
        Token::new(TokenKind::Sub, "[-]"),
        Token::new(TokenKind::Mul, "[*]"),
        Token::new(TokenKind::Div, "[/]"),
        Token::new(TokenKind::Sqrt, ".sqrt"),
        Token::new(TokenKind::Number, r"\d+"),
        Token::new(TokenKind::Argument, "x"),
    ];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TokenCapture<'a> {
    start: usize,
    end: usize,
    value: &'a str,
    kind: TokenKind,
}

impl<'a> TokenCapture<'a> {
    pub fn new(value: &str, kind: TokenKind) -> TokenCapture {
        TokenCapture {
            start: 0,
            end: 0,
            value,
            kind,
        }
    }
    pub fn kind(&self) -> TokenKind {
        self.kind
    }
    pub fn value(&self) -> &str {
        self.value
    }
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    regex: Regex,
}

impl Token {
    pub fn new(kind: TokenKind, re: &str) -> Token {
        Token {
            kind,
            regex: Regex::new(re).unwrap(),
        }
    }
    pub fn caputure_in_fn<'a>(&self, fn_str: &'a str, token_captures: &mut Vec<TokenCapture<'a>>) {
        for capture in self.regex.captures_iter(fn_str) {
            if let Some(capture) = capture.get(0) {
                token_captures.push(TokenCapture {
                    start: capture.start(),
                    end: capture.end(),
                    value: capture.as_str(),
                    kind: self.kind,
                })
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
    use crate::fun_eval::lexer::{find_tokens, TokenCapture, TokenKind};

    #[test]
    fn test_find_tokens() {
        //let input = "y = x^2 + x - sin(x) / e^x";
        let input = "x / (x+1)";
        let tokens = find_tokens(input);
        assert_eq!(
            tokens,
            vec![
                TokenCapture {
                    start: 0,
                    end: 1,
                    value: "x",
                    kind: TokenKind::Argument
                },
                TokenCapture {
                    start: 2,
                    end: 3,
                    value: "/",
                    kind: TokenKind::Div
                },
                TokenCapture {
                    start: 4,
                    end: 5,
                    value: "(",
                    kind: TokenKind::LeftParan
                },
                TokenCapture {
                    start: 5,
                    end: 6,
                    value: "x",
                    kind: TokenKind::Argument
                },
                TokenCapture {
                    start: 6,
                    end: 7,
                    value: "+",
                    kind: TokenKind::Add
                },
                TokenCapture {
                    start: 7,
                    end: 8,
                    value: "1",
                    kind: TokenKind::Number
                },
                TokenCapture {
                    start: 8,
                    end: 9,
                    value: ")",
                    kind: TokenKind::RightParan
                }
            ]
        );
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

    #[test]
    fn test_find_tokens_pow() {
        let input = "x ^ 3";
        let tokens = find_tokens(input);
        println!("tokens: {tokens:?}");
    }

    #[test]
    fn test_find_tokens_exp() {
        let input = "e ^ x";
        let tokens = find_tokens(input);
        println!("tokens: {tokens:?}");
    }

    #[test]
    fn test_find_tokens_sin() {
        let input = "(5*x).sin + 1";
        let tokens = find_tokens(input);
        for token in &tokens {
            println!("{}", token.kind.is_unary());
        }
        println!("tokens: {tokens:?}");
    }
}
