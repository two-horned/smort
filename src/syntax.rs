use std::collections::BTreeMap;

impl Default for Syntax {
    fn default() -> Self {
        Self::new()
    }
}

impl Syntax {
    pub fn new() -> Syntax {
        let mapping = BTreeMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
        Syntax {
            mapping,
            opened: vec![],
        }
    }

    pub fn opened(&self) -> &[char] {
        &self.opened
    }
    pub fn check_char(&mut self, c: char) -> bool {
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
    pub fn check_line(&mut self, line: &[char]) -> Option<char> {
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
