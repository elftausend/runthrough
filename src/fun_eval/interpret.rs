use std::fmt::Display;

use super::lexer::{self, TokenCapture, TokenKind};

#[derive(Debug)]
pub enum SyntaxError {
    Default,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Syntax Error")
    }
}

impl std::error::Error for SyntaxError {}

pub fn set_minus(tokens: &mut Vec<TokenCapture>) -> Vec<*mut str> {
    let mut string_handles = vec![];
    for token in tokens {
        if token.kind() != TokenKind::Sub {
            continue;
        }
        *token.kind_mut() = TokenKind::Add;
        let mut updated_value = token.value().to_string();
        if updated_value.starts_with('-') {
            updated_value.remove(0);
        } else {
            updated_value.insert(0, '-')
        }

        let updated_value = Box::leak(updated_value.into_boxed_str());

        token.set_value(updated_value);
        string_handles.push(updated_value as &str as *const str as *mut str);
    }
    string_handles
}

// TODO: notice '-' as unary and binary op
pub fn postfix_notation(
    tokens: Vec<TokenCapture>,
) -> Result<Vec<TokenCapture>, Box<dyn std::error::Error>> {
    //let string_handles = set_minus(&mut tokens);
    let mut stack = Vec::<TokenCapture>::new();
    let mut queue = Vec::<TokenCapture>::new();

    for token in tokens {
        if !token.kind().is_op() || token.kind().is_unary() {
            queue.push(token);
        } else if let Some(last) = stack.last() {
            if ((token.kind() == TokenKind::Add
                || token.kind() == TokenKind::Sub
                || token.kind() == TokenKind::Mul
                || token.kind() == TokenKind::Div)
                && last.kind() == TokenKind::Pow)
                || (token.kind() == TokenKind::Add || token.kind() == TokenKind::Sub)
                    && (last.kind() == TokenKind::Mul || last.kind() == TokenKind::Div)
            {
                queue.push(stack.pop().unwrap());
                stack.push(token);
            } else if token.kind() == TokenKind::RightParan {
                let mut popped = stack.pop().unwrap();
                while popped.kind() != TokenKind::LeftParan {
                    queue.push(popped);
                    popped = stack.pop().ok_or(SyntaxError::Default)?;
                }

                /*if stack[stack.len()-1].kind().is_unary() {
                    //println!("pushed");
                    queue.push(stack.pop().unwrap());
                }*/
            } else if !token.kind().is_unary() {
                stack.push(token);
            }
        } else {
            stack.push(token);
        }
    }

    for element in stack.into_iter().rev() {
        queue.push(element);
    }
    Ok(queue)
}

fn arg_or_num(token: &TokenCapture, x_populate: f64) -> f64 {
    if token.kind() == TokenKind::Argument {
        x_populate
    } else if token.kind() == TokenKind::E {
        std::f64::consts::E
    } else {
        token.value().parse::<f64>().unwrap()
    }
}

// TODO: rewrite
pub fn postfix_eval(
    postfix: &Vec<TokenCapture>,
    x_populate: f64,
) -> Result<f64, Box<dyn std::error::Error>> {
    let mut string_results = Vec::<*mut str>::new();
    let mut stack = Vec::<TokenCapture>::new();

    for token in postfix {
        if !token.kind().is_op() {
            stack.push(*token);
        } else if token.kind().is_unary() {
            let value = stack.pop().ok_or(SyntaxError::Default)?;
            let value = arg_or_num(&value, x_populate);
            let out = match token.kind() {
                TokenKind::Sin => value.sin(),
                TokenKind::Cos => value.cos(),
                TokenKind::Sqrt => value.sqrt(),
                _ => 0.,
            };
            let x = Box::leak(out.to_string().into_boxed_str());
            stack.push(TokenCapture::new(x, TokenKind::Number));
            string_results.push(x as &str as *const str as *mut str);
        } else {
            let right = stack.pop().ok_or(SyntaxError::Default)?;

            // unwrapping means that a binary operator has a lhs and a rhs.
            let left = stack.pop().ok_or(SyntaxError::Default)?;

            let rhs = arg_or_num(&right, x_populate);
            let lhs = arg_or_num(&left, x_populate);

            let out = match token.kind() {
                TokenKind::Add => lhs + rhs,
                TokenKind::Mul => lhs * rhs,
                TokenKind::Sub => lhs - rhs,
                TokenKind::Div => lhs / rhs,
                TokenKind::Pow => lhs.powf(rhs),
                TokenKind::E => lhs.powf(rhs),
                _ => 0.,
            };
            let x = Box::leak(out.to_string().into_boxed_str());
            stack.push(TokenCapture::new(x, TokenKind::Number));
            string_results.push(x as &str as *const str as *mut str);
        }
    }

    // the argument is not populated with a value if only "x" was inputted
    let first = stack.first().ok_or(SyntaxError::Default)?;
    if first.kind() == TokenKind::Argument {
        return Ok(x_populate);
    }
    let output = first.value().to_string();

    for string in string_results {
        unsafe {
            Box::from_raw(string);
        }
    }
    Ok(output.parse::<f64>()?)
}

pub fn interpret_fn(input: &str) -> Result<Vec<TokenCapture>, Box<dyn std::error::Error>> {
    let tokens = lexer::find_tokens(input);
    postfix_notation(tokens)
}

#[cfg(test)]
mod tests {
    use super::{postfix_eval, postfix_notation};
    use crate::fun_eval::lexer;

    fn roughly_equals(a: f64, b: f64) {
        let diff = (a - b).abs();
        if diff >= 0.01 {
            panic!("a {a} != b {b}");
        }
    }

    #[test]
    fn test_interpret_tokens() -> Result<(), Box<dyn std::error::Error>> {
        let input = "(3+x pow 4)-1";

        let tokens = lexer::find_tokens(input);
        let postfix = postfix_notation(tokens)?;
        let output = postfix_eval(&postfix, 5.);
        assert_eq!((3. + 5f64.powf(4.)) - 1., output?);
        Ok(())
    }

    #[test]
    fn test_interpret_tokens_e() -> Result<(), Box<dyn std::error::Error>> {
        let input = "(3+e pow x)-1";

        let tokens = lexer::find_tokens(input);
        let postfix = postfix_notation(tokens)?;
        let output = postfix_eval(&postfix, 5.)?;
        roughly_equals(output, 150.413159);
        Ok(())
    }

    #[test]
    fn test_interpret_tokens_sin() -> Result<(), Box<dyn std::error::Error>> {
        let input = "x.sin";

        let tokens = lexer::find_tokens(input);
        let postfix = postfix_notation(tokens)?;
        let output = postfix_eval(&postfix, std::f64::consts::PI / 2.);

        assert_eq!(output?, 1.);
        Ok(())
    }

    #[test]
    fn test_interpret_tokens_sin1() -> Result<(), Box<dyn std::error::Error>> {
        let input = "(5*x).sin + (2 * 3).sin";

        let tokens = lexer::find_tokens(input);
        let postfix = postfix_notation(tokens)?;
        let output = postfix_eval(&postfix, std::f64::consts::PI / 2.);

        roughly_equals(output?, 0.7205845);
        Ok(())
    }

    #[test]
    fn test_interpret_tokens_parans() -> Result<(), Box<dyn std::error::Error>> {
        let input = "(((x + 1)pow 3) / 3) * 2";

        let tokens = lexer::find_tokens(input);
        let postfix = postfix_notation(tokens)?;
        let output = postfix_eval(&postfix, 2.);
        assert_eq!(output?, 18.);
        Ok(())
    }

    #[test]
    fn test_interpret_tokens_sqrt() -> Result<(), Box<dyn std::error::Error>> {
        let input = "((((x + 1)pow 3) / 3) * 2).sqrt";

        let tokens = lexer::find_tokens(input);
        let postfix = postfix_notation(tokens)?;
        let output = postfix_eval(&postfix, 2.);

        roughly_equals(output?, 4.2426);
        Ok(())
    }

    #[test]
    fn test_interpret_tokens_pow() -> Result<(), Box<dyn std::error::Error>> {
        let input = "xpow2 + 3";

        let tokens = lexer::find_tokens(input);
        let postfix = postfix_notation(tokens)?;
        let output = postfix_eval(&postfix, 2.);

        assert_eq!(7., output?);
        Ok(())
    }
}
