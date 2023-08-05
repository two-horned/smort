use crate::fraction::Fraction;

use crate::syntax::Syntax;
pub const NORMAL_OPERATORS: &str = "+*/%^!";
pub const OPERATORS: &str = "+-*/%!^()!";
pub const NUMBERS: &str = "1234567890.";

pub fn fraction_to_float(f: Fraction<isize>) -> f64 {
    f.numerator() as f64 / f.dividor() as f64
}

fn string_to_fraction(s: String) -> Result<Fraction<isize>, CalculatorError> {
    let mut l = 0;
    let mut s = s;
    if s.contains(".") {
        let (_,b) = s.split_once(".").unwrap();
        if b.contains(".") {
            CalculatorError::InvalidInputError(s.clone());
        }
        l = b.len();
        s.retain(|c| c != '.');
    }
    let n = s.parse().unwrap();
    let d = 10_isize.pow(l.try_into().unwrap());
    let f = Fraction::new(n,d).unwrap();
    Ok(f)
}

fn factorial(n: isize) -> isize {
    match n {
        0 | 1 => 1,
        _ => factorial(n - 1) * n,
    }
}

pub fn calculate(e: &str) -> Result<Fraction<isize>, CalculatorError> {
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

fn calc(e: &str) -> Result<Fraction<isize>, CalculatorError> {
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
        return string_to_fraction(e.to_string());
    };
    let (mut a, mut b) = e.split_once(o).unwrap();
    if o == '(' || o == ')' || o == '!' {
        if o == '(' {
            if a.is_empty() {
                return calc(b);
            }
            o = a.chars().last().unwrap();
            a = a.strip_suffix(o).unwrap();

        } else if o == ')' {
            if b.is_empty() {
                return calc(a);
            }
            o = b.chars().next().unwrap();
            b = b.strip_prefix(o).unwrap();
        } else if o == '!' {
            if a.is_empty() {
                return Err(CalculatorError::InvalidInputError(String::from(b)));
            }
            let a = factorial(a.parse().unwrap());
            let a = Fraction::new(a, 1).unwrap();
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
        Fraction::new(0, 1).unwrap()
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
            '/' => {
                if b == b + b {
                    return Err(CalculatorError::DivideWithZeroError);
                } a /= b},
            '*' => a *= b,
            _ => unreachable!("you reached supposedly unreachable code"),
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
