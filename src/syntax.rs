use std::collections::BTreeMap;
const ALLOWED: &str = "1234567890+-*/%^()!. ";

pub fn is_legal(e: &str) -> bool {
    if e.chars().all(|c| ALLOWED.contains(c)) {
        let mut syntax = Syntax::new();
        for c in e.chars() {
            if c == '('|| c == ')' {
                if !syntax.check_char(c) {
                    return false;
                }
            }
        }
        if !syntax.opened.is_empty() {
            return false;
        }
        return true;
    }
    false
}

impl Syntax {
    pub fn new() -> Syntax {
    let mapping = BTreeMap::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<')
    ]);
    Syntax { mapping: mapping, opened: vec![] }
    }

    pub fn check_char(&mut self, c: char)  -> bool {
        if self.mapping.contains_key(&c) {
            if self.mapping.get(&c) == self.opened.last() {
                self.opened.pop();
            } else {
                return false;
            }
        } else {
            self.opened.push(c);
        }
        true
    }

    pub fn check_line(&mut self, line: &Vec<char>) -> Option<char> {
        for l in line {
            if !self.check_char(*l) {
                return Some(*l);
            }
        }
        None
    }

    pub fn clear_opened(&mut self) -> Vec<char> {
        let opened = self.opened.to_vec();
        self.opened = vec![];
        opened
    }
}

pub struct Syntax {
    mapping: BTreeMap<char, char>,
    opened: Vec<char>,
}
