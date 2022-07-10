use super::lexer::{TokenCapture, TokenKind};

pub fn postfix_notation(tokens: Vec<TokenCapture>) -> Vec<TokenCapture> {
    let mut stack = Vec::<TokenCapture>::new();
    let mut queue = Vec::<TokenCapture>::new();

    for token in tokens {
        if !token.kind().is_op() {
            queue.push(token);
        } else if token.kind().is_unary() {
            queue.push(token);
        } else if let Some(last) = stack.last() {
            if token.kind() == TokenKind::Pow
                && (last.kind() == TokenKind::Mul || last.kind() == TokenKind::Div)
            {
                queue.push(stack.pop().unwrap());
                stack.push(token);
            } else if (token.kind() == TokenKind::Add || token.kind() == TokenKind::Sub)
                && (last.kind() == TokenKind::Mul || last.kind() == TokenKind::Div)
            {
                queue.push(stack.pop().unwrap());
                stack.push(token);
            } else if token.kind() == TokenKind::RightParan {
                let mut popped = stack.pop().unwrap();
                while popped.kind() != TokenKind::LeftParan {
                    queue.push(popped);
                    popped = stack.pop().unwrap();
                }

                /*if stack[stack.len()-1].kind().is_unary() {
                    //println!("pushed");
                    queue.push(stack.pop().unwrap());
                }*/
                
            } else {
                if !token.kind().is_unary() {
                    stack.push(token);
                }
                
            }
        } else {
            stack.push(token);
        }
    }

    for element in stack.into_iter().rev() {
        queue.push(element);
    }

    queue
}

fn arg_or_num(token: &TokenCapture, x_populate: f32) -> f32 {
    if token.kind() == TokenKind::Argument {
        x_populate
    } else if token.kind() == TokenKind::E {
        std::f32::consts::E
    } else {
        token.value().parse::<f32>().unwrap()
    }
}

pub fn postfix_eval(postfix: Vec<TokenCapture>, x_populate: f32) -> String {
    let mut string_results = Vec::<*mut str>::new();
    let mut stack = Vec::<TokenCapture>::new();

    for token in postfix {
        if !token.kind().is_op() {
            stack.push(token);
        } else {
            if token.kind().is_unary() {
                let value = stack.pop().unwrap();
                let value = arg_or_num(&value, x_populate);
                let out = match token.kind() {
                    TokenKind::Sin => value.sin(),
                    TokenKind::Sqrt => value.sqrt(),
                    _ => 0.,
                };
                let x = Box::leak(out.to_string().into_boxed_str());
                stack.push(TokenCapture::new(x, TokenKind::Number));
                string_results.push(x as &str as *const str as *mut str);
            } else {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

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
    }

    let output = stack[0].value().to_string();

    for string in string_results {
        unsafe {
            Box::from_raw(string);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::fun_eval::lexer;

    use super::{postfix_eval, postfix_notation};

    fn roughly_equals(a: f32, b: f32) {
        let diff = (a - b).abs();
        if diff >= 0.01 {
            panic!("a {a} != b {b}");
        }
    }

    #[test]
    fn test_interpret_tokens() {
        let input = "(3+x^4)-1";

        let tokens = lexer::find_tokens(input);
        let postfix = postfix_notation(tokens);
        let output = postfix_eval(postfix, 5.);
        assert_eq!(Ok((3. + 5f32.powf(4.)) - 1.), output.parse::<f32>());
    }

    #[test]
    fn test_interpret_tokens_e() {
        let input = "(3+e^x)-1";

        let tokens = lexer::find_tokens(input);
        let postfix = postfix_notation(tokens);
        let output = postfix_eval(postfix, 5.).parse::<f32>().unwrap();
        roughly_equals(output, 150.413159);
    }

    #[test]
    fn test_interpret_tokens_sin() {
        let input = "x.sin";

        let tokens = lexer::find_tokens(input);
        let postfix = postfix_notation(tokens);
        let output = postfix_eval(postfix, std::f32::consts::PI / 2.);

        assert_eq!(output.parse::<f32>().unwrap(), 1.);
    }

    #[test]
    fn test_interpret_tokens_sin1() {
        let input = "(5*x).sin + (2 * 3).sin";

        let tokens = lexer::find_tokens(input);
        let postfix = postfix_notation(tokens);
        let output = postfix_eval(postfix, std::f32::consts::PI / 2.);

        roughly_equals(output.parse::<f32>().unwrap(), 0.7205845);
    }

    #[test]
    fn test_interpret_tokens_parans() {
        let input = "(((x + 1)^3) / 3) * 2";

        let tokens = lexer::find_tokens(input);
        let postfix = postfix_notation(tokens);
        let output = postfix_eval(postfix, 2.);
        assert_eq!(output.parse::<f32>().unwrap(), 18.);
    }

    #[test]
    fn test_interpret_tokens_sqrt() {
        let input = "((((x + 1)^3) / 3) * 2).sqrt";

        let tokens = lexer::find_tokens(input);
        let postfix = postfix_notation(tokens);
        let output = postfix_eval(postfix, 2.);

        roughly_equals(output.parse::<f32>().unwrap(), 4.2426)
    }
}
