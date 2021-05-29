/// # Rectangel example from variouse sources.
/// <https://doc.rust-lang.org/rust-by-example/fn/methods.html>
/// Refactor to use point
///
#[derive(Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    /// static method often are constructors of some sort.
    /// create a point repesenting x: 0, y :0 on a graph.
    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    /// constructor for point using struct short hand
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

#[derive(Debug, PartialEq)]
struct Rectangle {
    v1: Point,
    v2: Point,
}

impl Rectangle {
    pub fn new(v1: Point, v2: Point) -> Rectangle {
        Rectangle { v1, v2 }
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width() > other.width() && self.height() > other.height()
    }

    fn width(&self) -> f64 {
        self.v1.x.abs() + self.v2.x.abs()
    }

    fn height(&self) -> f64 {
        self.v1.y.abs() + self.v2.y.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_data() -> (Rectangle, Rectangle) {
        (
            Rectangle::new(Point::new(9_f64, 0.0), Point::new(12.0, -8.0)),
            Rectangle::new(Point::new(1.0, 1.0), Point::new(2.0, 2.0)),
        )
    }

    #[test]
    fn larger_can_hold_smaller() {
        let recs = create_test_data();
        assert!((recs.0).can_hold(&(recs.1)));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let recs = create_test_data();
        assert!(!(recs.1).can_hold(&(recs.0)));
    }
}
