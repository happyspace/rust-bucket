use std::fmt;

#[derive(Debug)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

// create a display
impl<T> fmt::Display for Complex<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + i {}", self.re, self.im)
    }
}
