use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the Point type.

impl<T> fmt::Display for Point<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "x:{} y:{}", self.x, self.y)
    }
}
