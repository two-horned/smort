use std::boxed::Box;

use crate::fraction::Fraction;

impl Number {
    pub fn new(var: Box<str>, value: Fraction<f32>, exponent: Box<Number>) -> Self {
        Self {var, value, exponent}
    }

    pub fn var(self) -> Box<str> {
        self.var
    }

    pub fn value(self) -> Fraction<f32> {
        self.value
    }

    pub fn exponent(self) -> Box<Number> {
        self.exponent
    }
}

pub struct Number {
    var: Box<str>,
    value: Fraction<f32>,
    exponent: Box<Number>,
}
