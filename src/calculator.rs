const OPERATORS: &str = "+-*/%!^()";
fn splitter(e: &str, s: char) -> [&str; 2] {
    let vec = e.split_once(s).unwrap();
    [vec.0, vec.1]
}
fn pow(a: &str, b: &str) -> f64 {
    let a = calculate(a);
    let b = calculate(b);
    a.powf(b)
}

fn modo(a: &str, b: &str) -> f64 {
    let a = calculate(a);
    let b = calculate(b);
    a % b
}

fn dev(a: &str, b: &str) -> f64 {
    let a = calculate(a);
    let b = calculate(b);
    a / b
}

fn mul(a: &str, b: &str) -> f64 {
    let a = calculate(a);
    let b = calculate(b);
    a * b
}

fn sep_sub(a: &str, b: &str) -> f64 {
    let a = calculate(a);
    let b = calculate(b);
    a - b
}

fn sub(a: &str, b: &str) -> f64 {
    let a = calculate(a);
    let b = {
        let b: String = b.chars().map(|c| match c { '-' => '+', '+' => '-', _ => c, }).collect();
        calculate(&b)
    };
    a - b
}

fn add(a: &str, b: &str) -> f64 {
    let a = calculate(a);
    let b = calculate(b);
    a + b
}

fn seperator(e: &str) -> f64 {
    let [a, b] = splitter(e, '(');
    if a.len() != 0 {
        let operant = a.chars().last().unwrap();
        match operant {
            '+' => return add(a, b),
            '-' => return sep_sub(a, b),
            '/' => return dev(a, b),
            '%' => return modo(a, b),
            '^' => return pow(a, b),
            _ => return mul(a, b),
        }
    } else {
        return calculate(b);
    }
}

pub fn is_two_signs(e: &str) -> bool {
    if e.contains("--")|| e.contains("-+")|| e.contains("+-") ||e.contains("++") {
        return true;
    }
    false
}

pub fn calculate(e: &str) -> f64 {
    let mut ec = e.chars();
    if ec.all(|c| !OPERATORS.contains(c)) && !e.is_empty() {
        let solution: f64 = e.parse().expect("Parsing the given String failed.");
        return solution;
    } else if e.is_empty() {
        return 0.0;
    }

    if e.contains('(') {
        return seperator(e);
    } else if e.contains(')') {
        return calculate(splitter(e, ')')[0]);
    } else if e.contains('%') {
        let [a, b] = splitter(e, '%');
        return modo(a, &b);
    } else if e.contains('+') {
        let [a, b] = splitter(e, '+');
        return add(a, b);
    } else if e.contains('-') {
        let [a, b] = splitter(e, '-');
        return sub(a, b);
    } else if e.contains('/') {
        let [a, b] = splitter(e, '/');
        return dev(a, b);
    } else if e.contains('*') {
        let [a, b] = splitter(e, '*');
        return mul(a, b);
    } else if e.contains('^') {
        let [a, b] = splitter(e, '^');
        return pow(a, b);
    }
    panic!("No such operation found.")
}

