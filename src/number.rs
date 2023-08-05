impl<T> Number<T> {
    pub fn new(re: T, im: T) -> Self {
        Self {re, im}
    }
}

pub struct Number<T> {
    re: T,
    im: T
}
