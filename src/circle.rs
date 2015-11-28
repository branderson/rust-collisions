pub struct Circle {
    x: f32,
    y: f32,
    r: f32
}

impl Circle {
    /// Constructs a new Circle with given position and radius
    ///
    /// # Panics
    /// Radius must be nonnegative
    pub fn new(x_pos: f32, y_pos: f32, radius: f32) -> Circle {
        assert!(radius >= 0.0);

        Circle {
            x: x_pos,
            y: y_pos,
            r: radius,
        }
    }

    pub fn set_position(&mut self, x: f32, y:f32) {
        self.x = x;
        self.y = y;
    }

    /// Sets the radius of the circle.
    ///
    /// # Panics
    /// Radius must be nonnegative
    pub fn set_radius(&mut self, radius: f32) {
        assert!(radius >= 0.0);

        self.r = radius;
    }

    pub fn get_position(&self) -> (f32, f32) {
        return (self.x, self.y)
    }

    pub fn get_radius(&self) -> f32 {
        return self.r;
    }

    /// Checks if a circle collides with the circle. Returns true if circles collide.
    ///
    /// # Panics
    /// Circle radii must be nonnegative
    pub fn check_collision(&self, other: &Circle) -> bool {
        // Make sure neither circle has a negative radius
        assert!(self.r >= 0.0 && other.r >= 0.0);

        // Find all relavant values and cast to f64
        let rad_difference: f64 = (self.r - other.r) as f64;
        let rad_sum: f64 = (self.r + other.r) as f64;
        let x_difference: f64 = (self.x - other.x) as f64;
        let y_difference: f64 = (self.y - other.y) as f64;

        // Find range of distances for which circles would collide
        let min_distance: f64 = rad_difference.powi(2);
        let max_distance: f64 = rad_sum.powi(2);
        let distance: f64 = x_difference.powi(2) + y_difference.powi(2);

        // Return whether distance is within colliding range
        return distance >= min_distance && distance <= max_distance;
    }
}

#[cfg(test)]
mod test {
    use super::Circle;

    #[test]
    fn test_new_circle_allowed() {
        Circle::new(0.0, 0.0, 10.0);
        Circle::new(0.0, 0.0, 0.0);
    }

    #[test]
    #[should_panic(expected = "assertion failed: radius >= 0.0")]
    fn test_new_circle_panic() {
        Circle::new(0.0, 0.0, -1.0);
    }

    #[test]
    fn test_circle_position() {
        let mut circle = Circle::new(0.0, 0.0, 0.0);
        assert_eq!(circle.get_position(), (0.0, 0.0));
        circle.set_position(10.0, -10.0);
        assert_eq!(circle.get_position(), (10.0, -10.0));
    }

    #[test]
    fn test_set_circle_radius_allowed() {
        let mut circle = Circle::new(0.0, 0.0, 0.0);
        assert_eq!(circle.get_radius(), 0.0);
        circle.set_radius(10.0);
        assert_eq!(circle.get_radius(), 10.0);
    }

    #[test]
    #[should_panic(expected = "assertion failed: radius >= 0.0")]
    fn test_set_circle_radius_panic() {
        let mut circle = Circle::new(0.0, 0.0, 0.0);
        circle.set_radius(-10.0);
    }

    #[test]
    fn test_check_circle_collision() {
        let mut circle1 = Circle::new(0.0, 0.0, 10.0);
        let mut circle2 = Circle::new(0.0, 0.0, 10.0);
        assert_eq!(circle1.check_collision(&circle2), true);
        assert_eq!(circle2.check_collision(&circle1), true);
        circle2.set_position(20.1, 0.0);
        // circle2.x = 20.1;
        // circle2.y = 0.0;
        assert_eq!(circle1.check_collision(&circle2), false);
        circle1.set_position(0.0, -20.1);
        circle2.set_position(0.0, 0.0);
        // circle1.x = 0.0;
        // circle1.y = -20.1;
        // circle2.x = 0.0;
        // circle2.y = 0.0;
        assert_eq!(circle1.check_collision(&circle2), false);
    }
}

