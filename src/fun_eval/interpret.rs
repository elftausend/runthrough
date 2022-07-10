use super::lexer::{TokenCapture, TokenKind};

pub fn postfix_notation(tokens: Vec<TokenCapture>) -> Vec<TokenCapture> {
    let mut stack = Vec::<TokenCapture>::new();
    let mut queue = Vec::<TokenCapture>::new();

    for token in tokens {
        if !token.kind().is_op() {
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
            } else {
                stack.push(token);
            }
        } else {
            stack.push(token);
        }
    }
    queue.push(stack.pop().unwrap());
    queue
}

fn arg_or_num(token: &TokenCapture, x_populate: f32) -> f32 {
    if token.kind() == TokenKind::Argument {
        x_populate
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
                _ => 0.,
            };
            let x = Box::leak(out.to_string().into_boxed_str());
            stack.push(TokenCapture::new(x, TokenKind::Number));
            string_results.push(x as &str as *const str as *mut str);
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

    #[test]
    fn test_interpret_tokens() {
        let input = "(3+x^4)-1";
        //let input = "((1+4*3+8/3-2) / (20+5*2-5) + (2 * 5 + 10)) * (4 + 1 * 8 / 8)";
        // add input check
        let tokens = lexer::find_tokens(input);
        let postfix = postfix_notation(tokens);
        let output = postfix_eval(postfix, 5.);
        println!("out: {output}");
    }
}
