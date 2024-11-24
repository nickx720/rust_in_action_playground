enum Shape {
    Circle,
    Square,
    Rectangle,
    Triangle,
    Pentagon,
}

impl Shape {
    // TODO: Implement the `n_sides` method using a `match`.
    pub fn n_sides(&self) -> u8 {
        match self {
            Shape::Circle => 0u8,
            Shape::Square => 4u8,
            Shape::Rectangle => 4u8,
            Shape::Triangle => 3u8,
            Shape::Pentagon => 5u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        assert_eq!(Shape::Circle.n_sides(), 0);
    }

    #[test]
    fn test_square() {
        assert_eq!(Shape::Square.n_sides(), 4);
    }

    #[test]
    fn test_rectangle() {
        assert_eq!(Shape::Rectangle.n_sides(), 4);
    }

    #[test]
    fn test_triangle() {
        assert_eq!(Shape::Triangle.n_sides(), 3);
    }

    #[test]
    fn test_pentagon() {
        assert_eq!(Shape::Pentagon.n_sides(), 5);
    }
}
