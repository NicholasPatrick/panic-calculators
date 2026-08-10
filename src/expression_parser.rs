use malachite::base::num::arithmetic::traits::Factorial;
use malachite::base::num::arithmetic::traits::Parity;
use malachite::{
    Integer, Natural,
    base::num::{
        arithmetic::traits::{FloorSqrt, Pow, UnsignedAbs},
        basic::traits::One,
        conversion::traits::SaturatingInto,
    },
};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Num(String),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Fac,
    LPar,
    RPar,
    Comma,
    Func(String),
}

#[derive(Debug)]
enum Expression {
    Num(String),
    Add([Box<Expression>; 2]),
    Neg(Box<Expression>),
    Sub([Box<Expression>; 2]),
    Mul([Box<Expression>; 2]),
    Div([Box<Expression>; 2]),
    Mod([Box<Expression>; 2]),
    Pow([Box<Expression>; 2]),
    Fac(Box<Expression>),
    Isqrt(Box<Expression>),
}

fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    let mut partial = String::new();
    fn flush_partial(partial: &mut String, tokens: &mut Vec<Token>) {
        if partial.is_empty() {
            return;
        }
        if partial.starts_with(|c: char| c.is_alphabetic()) {
            tokens.push(Token::Func(std::mem::take(partial)));
        } else {
            tokens.push(Token::Num(std::mem::take(partial)));
        }
        partial.clear()
    }
    for c in s.chars() {
        if c.is_alphanumeric() || c == '.' {
            partial.push(c);
            continue;
        }
        flush_partial(&mut partial, &mut tokens);
        if c.is_whitespace() {
            continue;
        }
        tokens.push(match c {
            '+' => Token::Add,
            '-' => Token::Sub,
            '*' => Token::Mul,
            '/' => Token::Div,
            '%' => Token::Mod,
            '^' => Token::Pow,
            '!' => Token::Fac,
            '(' => Token::LPar,
            ')' => Token::RPar,
            ',' => Token::Comma,
            c => Result::Err(format!("Unknown operator {c}"))?,
        });
    }
    flush_partial(&mut partial, &mut tokens);
    Ok(tokens)
}

fn parse(tokens: &Vec<Token>) -> Result<Expression, String> {
    // this parser is not linear-time
    fn parse_slice(tokens: &[Token]) -> Result<Expression, String> {
        if tokens.len() == 0 {
            Err("Empty expression")?
        }
        if tokens.len() == 1 {
            return match &tokens[0] {
                Token::Num(n) => Ok(Expression::Num(n.to_string())),
                _ => Err("Malformed")?,
            };
        }
        let mut depth = 0;
        // match parens
        for i in 0..tokens.len() {
            match tokens[i] {
                Token::LPar => {
                    depth += 1;
                }
                Token::RPar => {
                    depth -= 1;
                    if depth < 0 {
                        Err("Unmatched )")?
                    }
                }
                _ => (),
            }
        }
        if depth != 0 {
            Err("Unmatched (")?
        }
        // search for +-
        for i in (0..tokens.len()).rev() {
            match tokens[i] {
                Token::LPar => depth += 1,
                Token::RPar => depth -= 1,
                Token::Add => {
                    if depth != 0 {
                        continue;
                    }
                    let right = parse_slice(&tokens[i + 1..])?;
                    if i == 0 {
                        return Ok(right);
                    }
                    let left = parse_slice(&tokens[..i])?;
                    return Ok(Expression::Add([Box::new(left), Box::new(right)]));
                }
                Token::Sub => {
                    if depth != 0 {
                        continue;
                    }
                    let right = parse_slice(&tokens[i + 1..])?;
                    if i == 0 {
                        return Ok(Expression::Neg(Box::new(right)));
                    }
                    let left = parse_slice(&tokens[..i])?;
                    return Ok(Expression::Sub([Box::new(left), Box::new(right)]));
                }
                _ => (),
            }
        }
        // search for */%
        for i in (0..tokens.len()).rev() {
            match tokens[i] {
                Token::LPar => depth += 1,
                Token::RPar => depth -= 1,
                Token::Mul => {
                    if depth != 0 {
                        continue;
                    }
                    let left = parse_slice(&tokens[..i])?;
                    let right = parse_slice(&tokens[i + 1..])?;
                    return Ok(Expression::Mul([Box::new(left), Box::new(right)]));
                }
                Token::Div => {
                    if depth != 0 {
                        continue;
                    }
                    let left = parse_slice(&tokens[..i])?;
                    let right = parse_slice(&tokens[i + 1..])?;
                    return Ok(Expression::Div([Box::new(left), Box::new(right)]));
                }
                Token::Mod => {
                    if depth != 0 {
                        continue;
                    }
                    let left = parse_slice(&tokens[..i])?;
                    let right = parse_slice(&tokens[i + 1..])?;
                    return Ok(Expression::Mod([Box::new(left), Box::new(right)]));
                }
                _ => (),
            }
        }
        // search for ^
        for i in 0..tokens.len() {
            match tokens[i] {
                Token::LPar => depth += 1,
                Token::RPar => depth -= 1,
                Token::Pow => {
                    if depth != 0 {
                        continue;
                    }
                    let left = parse_slice(&tokens[..i])?;
                    let right = parse_slice(&tokens[i + 1..])?;
                    return Ok(Expression::Pow([Box::new(left), Box::new(right)]));
                }
                _ => (),
            }
        }
        // search for !
        for i in (0..tokens.len()).rev() {
            match tokens[i] {
                Token::LPar => depth += 1,
                Token::RPar => depth -= 1,
                Token::Fac => {
                    if depth != 0 {
                        continue;
                    }
                    if i != tokens.len() - 1 {
                        Err("Invalid expression, operator missing")?
                    }
                    let left = parse_slice(&tokens[..i])?;
                    return Ok(Expression::Fac(Box::new(left)));
                }
                _ => (),
            }
        }
        // search for functions
        for i in 0..tokens.len() {
            match &tokens[i] {
                Token::LPar => depth += 1,
                Token::RPar => depth -= 1,
                Token::Func(s) => {
                    if depth != 0 {
                        continue;
                    }
                    if i != 0 {
                        Err("Invalid expression, operator missing")?
                    }
                    let right = parse_slice(&tokens[i + 1..])?;
                    if s == "isqrt" {
                        return Ok(Expression::Isqrt(Box::new(right)));
                    }
                    Err(format!("Invalid function name {s}"))?
                }
                _ => (),
            }
        }
        // unwrap parens
        if tokens.first() == Some(&Token::LPar) && tokens.last() == Some(&Token::RPar) {
            return parse_slice(&tokens[1..tokens.len() - 1]);
        }
        Err("No operations")?
    }
    parse_slice(&tokens)
}

fn evaluate(expression: &Expression) -> Result<Integer, String> {
    match expression {
        Expression::Num(s) => Integer::from_str(s)
            .map_err(|_| "Number literal can't be converted to an integer".to_string()),
        Expression::Add(operands) => Ok(evaluate(&operands[0])? + evaluate(&operands[1])?),
        Expression::Neg(operand) => Ok(-evaluate(&operand)?),
        Expression::Sub(operands) => Ok(evaluate(&operands[0])? - evaluate(&operands[1])?),
        Expression::Mul(operands) => Ok(evaluate(&operands[0])? * evaluate(&operands[1])?),
        Expression::Div(operands) => {
            let num = evaluate(&operands[0])?;
            let den = evaluate(&operands[1])?;
            if den == 0 {
                Err("Division by 0")?
            }
            Ok(num / den)
        }
        Expression::Mod(operands) => {
            let num = evaluate(&operands[0])?;
            let den = evaluate(&operands[1])?;
            if den == 0 {
                Err("Modulo by 0")?
            }
            Ok(num % den)
        }
        Expression::Pow(operands) => {
            let base = evaluate(&operands[0])?;
            let exp = evaluate(&operands[1])?;
            if base == 1 {
                return Ok(Integer::ONE);
            }
            if base == -1 {
                return Ok(if exp.even() {
                    Integer::ONE
                } else {
                    -Integer::ONE
                });
            }
            if exp < 0 {
                Err("Exponent is negative but |base| > 1")?
            }
            if exp >= 100000 {
                Err("Exponentiation result is too large (>= 2^100000)")?
            }
            if base.clone().unsigned_abs().approx_ln().ln() + exp.clone().unsigned_abs().approx_ln()
                > 11.146412544388564
            {
                Err("Exponentiation result is too large (>= 2^100000)")?
            }
            Ok(base.pow((&exp.unsigned_abs()).saturating_into()))
        }
        Expression::Fac(operand) => {
            let operand = evaluate(&operand)?;
            if operand < 0 {
                Err("factorial of a negative number")?
            }
            if operand >= 8600 {
                Err("factorial result is too large (>= 2^100000")?
            }
            Ok(Natural::factorial((&operand).saturating_into()).into())
        }
        Expression::Isqrt(operand) => {
            let operand = evaluate(operand)?;
            if operand < 0 {
                Err("isqrt of negative")?
            }
            Ok(operand.floor_sqrt())
        }
    }
}

pub fn parse_to_int(s: &str) -> Result<Integer, String> {
    let tokens = tokenize(s)?;
    let expression = parse(&tokens)?;
    evaluate(&expression)
}

pub fn parse_to_natural(s: &str) -> Result<Natural, String> {
    let n = parse_to_int(s)?;
    match Natural::try_from(n) {
        Ok(ok) => Ok(ok),
        Err(_) => Err("Not a natural".to_owned()),
    }
}
