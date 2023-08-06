use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign, Neg};
use std::str::FromStr;

impl<T> Number<T>
where T: Sub<Output = T>
{
    pub fn new(re: T, im: T) -> Self {
        Self {re, im}
    }

    fn zero(&self) -> T {
        self.re - self.re
    }

    fn is_zero(&self) -> T {
        self == Self{re: self.zero(), im: self.zero()}
    }

    fn one(&self) -> T {
        if 
    }
}

impl<T> Add for Number<T>
where T: Add<Output = T>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self{re: self.re + rhs.re, im: self.im + rhs.im}
    }
}

impl<T> AddAssign for Number<T>
where T: AddAssign
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Sub for Number<T>
where T: Sub<Output = T>
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self{re: self.re - rhs.re, im: self.im - rhs.im}
    }
}

impl<T> SubAssign for Number<T>
where T: SubAssign 
{
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

impl<T> Neg for Number<T>
where T: Neg<Output = T>
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self{re: -self.re, im: -self.im}
    }
}

impl<T> Mul for Number<T>
where T: Mul<Output = T>
        + Add<Output = T> 
        + Sub<Output = T>
        + Copy
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self{
            re: self.re * rhs.re - self.im * rhs.im, 
            im: self.re * rhs.im + self.im * rhs.re
        }
    }
}

impl<T> MulAssign for Number<T>
where T: Mul<Output = T> 
        + Add<Output = T> 
        + Sub<Output = T>
        + Copy
{
    fn mul_assign(&mut self, rhs: Self) {
        let re = self.re * rhs.re - self.im * rhs.im;
        let im = self.re * rhs.im + self.im * rhs.re;
        self.re = re;
        self.im = im;
    }
}

impl<T> Div for Number<T>
where T: Mul<Output = T>
        + Div<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Copy
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let div = rhs.re * rhs.re + rhs.im * rhs.im;
        Self{
            re: (self.re * rhs.re + self.im * rhs.im) / div,
            im: (self.im * rhs.re - self.re * rhs.im) / div
        }
    }
}

impl<T> DivAssign for Number<T>
where T: Mul<Output = T>
        + Div<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Copy
{
    fn div_assign(&mut self, rhs: Self) {
        let div = rhs.re * rhs.re + rhs.im * rhs.im;
        let re = (self.re * rhs.re + self.im * rhs.im) / div;
        let im = (self.im * rhs.re - self.re * rhs.im) / div;
        self.re = re;
        self.im = im;
    }
}

impl<T> Rem for Number<T>
where T: Mul<Output = T>
        + Div<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Copy
{
    fn rem(self, rhs: Self) -> Self::Output {
    }
}

impl<T> FromStr for Number<T>
where T: Add<Output = T> + Sub<Output = T> + FromStr + Copy
{
    //type Output = Self;
    type Err = ParseNumberError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.to_string();
        s.retain(|c| c != ' ');
        if s.is_empty() {
            return Err(ParseNumberError(s));
        }

        let o = if s.contains('+') { '+' } else if s.contains('-') { '-' } else if s.contains('i') { 'i' } else { 'n' };
        if o == 'n' {
            match T::from_str(&s) {
                Ok(s) => return Ok(Self{re: s, im: s - s}),
                _ => return Err(ParseNumberError(s))
            }
        }
        if o == 'i' {
            s.retain(|c| c != 'i');
            match T::from_str(&s) {
                Ok(s) => return Ok(Self{re: s - s, im: s}),
                _ => return Err(ParseNumberError(s))
            }
        }

        let (re, im) = s.split_once(o).unwrap();
        match o {
            '+' => return Ok(Self::from_str(re)? + Self::from_str(im)?),
            '-' => return Ok(Self::from_str(re)? - Self::from_str(im)?),
            _ => unreachable!("you reached supposedly unreachable code")
        }
    }
}

#[derive(Debug)]
pub struct ParseNumberError(String);

#[derive(Clone, Copy, PartialEq)]
pub struct Number<T> {
    re: T,
    im: T
}
