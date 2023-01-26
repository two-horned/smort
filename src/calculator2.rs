use crate::fraction::Fraction;

use crate::syntax::Syntax;
pub const NORMAL_OPERATORS: &str = "+*/%^!";
pub const OPERATORS: &str = "+-*/%!^()!";
pub const NUMBERS: &str = "1234567890.";

pub fn fraction_to_float(f: Fraction<f32>) -> f32 {
    f.numerator() / f.dividor()
}

fn factorial(n: isize) -> isize {
    match n {
        0 | 1 => 1,
        _ => factorial(n - 1) * n,
    }
}

pub fn calculate(e: &str) -> Result<Fraction<f32>, CalculatorError> {
    if e.is_empty() {
        return Err(CalculatorError::EmptyInputError);
    } else if e
        .chars()
        .zip(e.chars().skip(1))
        .any(|(a, b)| NORMAL_OPERATORS.contains(a) && NORMAL_OPERATORS.contains(b))
    {
        return Err(CalculatorError::InvalidInputError(e.to_string()));
    } else if e
        .chars()
        .any(|c| !OPERATORS.contains(c) && !NUMBERS.contains(c))
    {
        return Err(CalculatorError::InvalidInputError(String::from(e)));
    } else if e.chars().all(|c| OPERATORS.contains(c)) {
        return Err(CalculatorError::NoNumbersGivenError);
    }
    let mut syntax = Syntax::new();
    for c in e.chars() {
        if (c == '(' || c == ')') && !syntax.check_char(c) {
            return Err(CalculatorError::InvalidParenthesisError);
        }
    }
    if !syntax.opened().is_empty() {
        return Err(CalculatorError::InvalidParenthesisError);
    }
    Ok(calc(e)?)
}

fn calc(e: &str) -> Result<Fraction<f32>, CalculatorError> {
    let mut o = if e.contains('(') {
        '('
    } else if e.contains(')') {
        ')'
    } else if e.contains('+') {
        '+'
    } else if e.contains('-') {
        '-'
    } else if e.contains('*') {
        '*'
    } else if e.contains('/') {
        '/'
    } else if e.contains('%') {
        '%'
    } else if e.contains('^') {
        '^'
    } else if e.contains('!') {
        '!'
    } else {
        return match e.parse() {
            Ok(i) => Ok(Fraction::new(i, 1.0).unwrap()),
            _ => Err(CalculatorError::InvalidInputError(String::from(e))),
        };
    };
    if o == '(' || o == ')' || o == '!' {
        let (a, b) = e.split_once(o).unwrap();
        if o == '(' {
            if a.is_empty() {
                return calc(b);
            }
            o = a.chars().last().unwrap();
        } else if o == ')' {
            if b.is_empty() {
                return calc(a);
            }
            o = b.chars().next().unwrap();
        } else if o == '!' {
            if a.is_empty() {
                return Err(CalculatorError::InvalidInputError(String::from(b)));
            }
            let a = factorial(a.parse().unwrap());
            let a = Fraction::new(a as f32, 1.0).unwrap();
            if b.is_empty() {
                return Ok(a);
            }
            o = '*';
        }
        let a = calc(a)?;
        let b = calc(b)?;
        return Ok(match o {
            '+' => a + b,
            '-' => a - b,
            '%' => a % b,
            '^' => a.powi(b.numerator() as i32),
            '/' => a / b,
            _ => a * b,
        });
    }
    let mut s = e.split(o);
    let a = s.next().unwrap();
    let mut a = if a.is_empty() {
        Fraction::new(0.0, 1.0).unwrap()
    } else {
        calc(a)?
    };
    for b in s {
        let b = calc(b)?;
        match o {
            '+' => a += b,
            '-' => a -= b,
            '%' => a %= b,
            '^' => a = a.powi(b.numerator() as i32),
            '/' => a /= b,
            '*' => a *= b,
            _ => panic!("WHAAATT!?!"),
        };
    }
    Ok(a)
}

#[derive(Debug)]
pub enum CalculatorError {
    InvalidParenthesisError,
    EmptyInputError,
    NoNumbersGivenError,
    DivideWithZeroError,
    InvalidInputError(String),
}
