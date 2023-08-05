use {
    core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign, Neg},
    std::{
        fmt::{self, Display},
        str::FromStr,
    },
};

impl<T> Fraction<T>
where
    T: Add<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Mul<Output = T>
        + MulAssign
        + DivAssign
        + PartialEq
        + Copy,
{
    pub fn new(n: T, d: T) -> Result<Self, DivideWithZeroError> {
        if d == d + d {
            return Err(DivideWithZeroError);
        }

        Ok(Self { n, d }.simplify())
    }

    pub fn numerator(&self) -> T {
        self.n
    }

    pub fn dividor(&self) -> T {
        self.d
    }

    pub fn powi(&self, i: i32) -> Self {
        if *self * *self == *self {
            return *self;
        }
        let mut i = i;
        let mut me = *self;
        if i == 0 {
            let one = self.d / self.d;
            return Self { n: one, d: one };
        }
        while i > 1 {
            me *= *self;
            i -= 1;
        }
        while i < 0 {
            me /= *self;
            i += 1;
        }
        me.simplify()
    }

    fn simplify(&self) -> Self {
        if self.n == self.n + self.n {
            return Self {
                n: self.n,
                d: self.d / self.d,
            };
        }
        let mut g = self.n;
        let mut m = self.d % g;
        while m != m + m {
            let k = g;
            g = m;
            m = k % g;
        }
        Self {
            n: self.n / g,
            d: self.d / g,
        }
    }

    fn simplify_assign(&mut self) {
        if self.n != self.n + self.n && self.n != self.n * self.n && self.d != self.d * self.d {
            let mut g = self.n;
            let mut m = self.d % g;
            while m != m + m {
                let k = g;
                g = m;
                m = k % g;
            }
            self.n /= g;
            self.d /= g;
        }
    }
}

impl<T> Display for Fraction<T>
where
    T: Display + Mul<Output = T> + PartialEq + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        if self.d == self.d * self.d {
            return write!(f, "{}", self.n);
        }
        write!(f, "{}/{}", self.n, self.d)
    }
}

impl<T> FromStr for Fraction<T>
where
    T: FromStr
        + Add<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Mul<Output = T>
        + MulAssign
        + DivAssign
        + PartialEq
        + Copy,
{
    type Err = ParseFractionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (n, d) = match s.split_once('/') {
            Some((n, d)) => match (n.parse(), d.parse()) {
                (Ok(n), Ok(d)) => (n, d),
                _ => return Err(ParseFractionError(String::from(s))),
            },
            _ => return Err(ParseFractionError(String::from(s))),
        };
        if d == d + d {
            return Err(ParseFractionError(String::from(s)));
        }
        Ok(Self::new(n, d).unwrap())
    }
}

impl<T> Add for Fraction<T>
where
    T: Add<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + MulAssign
        + DivAssign
        + PartialEq
        + Copy,
{
    type Output = Fraction<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self{n: self.n * rhs.d + rhs.n * self.d, d: self.d * rhs.d}.simplify()
    }
}

impl<T> AddAssign for Fraction<T>
where
    T: AddAssign
        + Mul<Output = T>
        + MulAssign
        + Add<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + DivAssign
        + PartialEq
        + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.n *= rhs.d;
        self.n += rhs.n * self.d;
        self.d *= rhs.d;
        self.simplify_assign();
    }
}

impl<T> Sub for Fraction<T>
where
    T: Sub<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + MulAssign
        + DivAssign
        + PartialEq
        + Copy,
{
    type Output = Fraction<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.n * rhs.d - rhs.n * self.d, self.d * rhs.d).unwrap()
    }
}

impl<T> SubAssign for Fraction<T>
where
    T: SubAssign
        + MulAssign
        + Mul<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + DivAssign
        + PartialEq
        + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.n *= rhs.d;
        self.n -= rhs.n * self.d;
        self.d *= rhs.d;
        self.simplify_assign();
    }
}

impl<T> Mul for Fraction<T>
where
    T: Mul<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + MulAssign
        + DivAssign
        + PartialEq
        + Copy,
{
    type Output = Fraction<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.n * rhs.n, self.d * rhs.d).unwrap()
    }
}

impl<T> MulAssign for Fraction<T>
where
    T: MulAssign
        + Add<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Mul<Output = T>
        + DivAssign
        + PartialEq
        + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.n *= rhs.n;
        self.d *= rhs.d;
        self.simplify_assign();
    }
}

impl<T> Div for Fraction<T>
where
    T: Mul<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + MulAssign
        + DivAssign
        + PartialEq
        + Copy,
{
    type Output = Fraction<T>;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.n * rhs.d, self.d * rhs.n).unwrap()
    }
}

impl<T> DivAssign for Fraction<T>
where
    T: MulAssign
        + Add<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Mul<Output = T>
        + DivAssign
        + PartialEq
        + Copy,
{
    fn div_assign(&mut self, rhs: Self) {
        self.n *= rhs.d;
        self.d *= rhs.n;
        if self.d == self.d + self.d {
            panic!("Divion with Zero");
        }
        self.simplify_assign();
    }
}

impl<T> Rem for Fraction<T>
where
    T: Mul<Output = T>
        + Rem<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + MulAssign
        + DivAssign
        + PartialEq
        + Copy,
{
    type Output = Fraction<T>;
    fn rem(self, rhs: Self) -> Self::Output {
        Self::new(self.n * rhs.d % rhs.n * self.d, rhs.d).unwrap()
    }
}

impl<T> RemAssign for Fraction<T>
where
    T: MulAssign
        + RemAssign
        + Mul<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + DivAssign
        + PartialEq
        + Copy,
{
    fn rem_assign(&mut self, rhs: Self) {
        self.n *= rhs.d;
        self.n %= rhs.n * self.d;
        self.d = rhs.d;
        self.simplify_assign();
    }
}

impl<T> Neg for Fraction<T>
where
    T: Neg<Output = T>
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self{n: self.n.neg(), d: self.d}
    }
}

#[derive(Debug)]
pub struct ParseFractionError(String);

#[derive(Debug)]
pub struct DivideWithZeroError;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Fraction<T> {
    n: T,
    d: T,
}
