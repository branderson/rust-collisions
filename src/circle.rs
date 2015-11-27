pub struct Circle {
    x: f32,
    y: f32,
    r: f32
}

impl Circle {
    /// Constructs a new Circle with given radius
    ///
    /// # Panics
    /// Radius cannot be < 0
    fn new(radius: f32) -> Circle {
        if radius < 0.0 {
            panic!("Radius cannot be < 0");
        }

        Circle {
            x: 0.0,
            y: 0.0,
            r: radius,
        }
    }

    /// Checks if a circle collides with the circle
    pub fn check_collision(&self, other: &Circle) -> bool {
        let rad_difference: f64 = (self.r - other.r) as f64;
        let rad_sum: f64 = (self.r + other.r) as f64;
        let x_difference: f64 = (self.x - other.x) as f64;
        let y_difference: f64 = (self.y - other.y) as f64;
        let min_distance: f64 = rad_difference.powi(2);
        let max_distance: f64 = rad_sum.powi(2);
        let distance: f64 = x_difference.powi(2) + y_difference.powi(2);
        return distance >= min_distance && distance <= max_distance;
    }
}

#[cfg(test)]
mod test {
    use super::Circle;

    #[test]
    fn test_check_circle_collision() {
        let mut circle1 = Circle::new(10.0);
        let mut circle2 = Circle::new(10.0);
        assert_eq!(circle1.check_collision(&circle2), true);
        assert_eq!(circle2.check_collision(&circle1), true);
        circle2.x = 20.1;
        circle2.y = 0.0;
        assert_eq!(circle1.check_collision(&circle2), false);
        circle1.x = 0.0;
        circle1.y = -20.1;
        circle2.x = 0.0;
        circle2.y = 0.0;
        assert_eq!(circle1.check_collision(&circle2), false);
    }
}

