const OPERATORS: &'static str = "+-*/%!^()!";

fn swapper(s: &str, c: char, n: char) -> String {
    s.chars()
        .map(|f| {
            if f == c {
                n
            } else if f == n {
                c
            } else {
                f
            }
        })
        .collect()
}

fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => factorial(n - 1) * n,
    }
}

pub fn calculate(e: &str) -> Result<f64, CalculatorError> {
    if e.is_empty() {
        return Ok(0.0);
    } else if e.chars().all(|c| !OPERATORS.contains(c)) {
        return match e.parse() {
            Ok(e) => Ok(e),
            _ => Err(CalculatorError::InvalidInputError(e.to_string())),
        };
    }

    let mut o = if e.contains('(') {
        '('
    } else if e.contains(')') {
        ')'
    } else if e.contains('%') {
        '%'
    } else if e.contains('+') {
        '+'
    } else if e.contains('-') {
        '-'
    } else if e.contains('/') {
        '/'
    } else if e.contains('*') {
        '*'
    } else if e.contains('^') {
        '^'
    } else {
        '!'
    };
    let (mut a, mut b) = e.split_once(o).unwrap();
    let is_swap = o != '(';
    if a.is_empty() && b.is_empty() {
        return Ok(0.0);
    } else if o == '(' {
        if a.is_empty() {
            return calculate(b);
        }
        o = a.chars().last().unwrap();
        if OPERATORS.contains(o) {
            a = &a[0..a.len() - 1];
        }
    } else if o == ')' {
        if b.is_empty() {
            return calculate(a);
        }
        o = b.chars().next().unwrap();
        if OPERATORS.contains(o) {
            b = &b[1..b.len()];
        }
    }
    let b = if is_swap {
        match o {
            '/' => calculate(&swapper(b, '/', '*'))?,
            '-' => calculate(&swapper(b, '-', '+'))?,
            _ => calculate(b)?,
        }
    } else {
        calculate(b)?
    };
    if b == 0_f64 && o == '/' {
        return Err(CalculatorError::DivideWithZeroError);
    }
    let a = calculate(a)?;
    let ans = match o {
        ')' => a,
        '+' => a + b,
        '-' => a - b,
        '/' => a / b,
        '%' => a % b,
        '^' => a.powf(b),
        '!' => factorial(a as u64) as f64,
        _ => a * b,
    };
    Ok(ans)
}
/*
enum Operation {
    add,
    sub,
    mul,
    div,
    modu,
    pow,
    fac,
    open,
    close,
}
*/
#[derive(Debug)]
pub enum CalculatorError {
    DivideWithZeroError,
    InvalidInputError(String),
}
