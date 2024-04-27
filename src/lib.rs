// In src/lib.rs

/// Represents linear motion between two points in 3D space.
/// Calculates positions between the start and end points based on a defined number of steps.
pub struct LinearMotion {
    pub start_x: f64,
    pub start_y: f64,
    pub start_z: f64,
    pub end_x: f64,
    pub end_y: f64,
    pub end_z: f64,
}

impl LinearMotion {
    pub fn calculate_positions(&self) {
        let dx = self.end_x - self.start_x;
        let dy = self.end_y - self.start_y;
        let dz = self.end_z - self.start_z;
        let steps = (dx.hypot(dy).hypot(dz) * 10.0).ceil() as usize; // Increase steps by multiplying by 10 for better precision
        for i in 1..=steps {
            let t = i as f64 / steps as f64;
            let x = self.start_x + t * dx;
            let y = self.start_y + t * dy;
            let z = self.start_z + t * dz;
            println!("{:.2}, {:.2}, {:.2}", x, y, z);
        }
    }
}

/// Represents circular motion around a center point with a specific radius.
/// Calculates positions along the circumference at regular angle intervals.
pub struct CircularMotion {
    pub center_x: f64,
    pub center_y: f64,
    pub center_z: f64,
    pub radius: f64,
}



impl CircularMotion {
    // This method now accepts a `direction` parameter to handle rotation direction.
    pub fn calculate_positions(&self, direction: &str) {
        let steps = 360 / 5;  // Number of steps for a full circle
        for step in 0..steps {
            let angle_degrees = if direction == "CW" {
                -(step * 5) // Clockwise rotation
            } else {
                step * 5 // Counterclockwise rotation
            };
            let angle = angle_degrees as f64 * std::f64::consts::PI / 180.0;
            let x = self.center_x + self.radius * angle.cos();
            let y = self.center_y + self.radius * angle.sin();
            let z = self.center_z; // Z remains constant for a flat circular motion
            println!("Circular Position at step {}: {:.2}, {:.2}, {:.2}", step, x, y, z);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_motion_positions() {
        let motion = LinearMotion { start_x: 0.0, start_y: 0.0, start_z: 0.0, end_x: 10.0, end_y: 10.0, end_z: 10.0 };
        motion.calculate_positions(); // This should call your method and suppress the dead_code warning
        // Assert results if necessary
    }

    #[test]
    fn test_circular_motion_positions_cw() {
        let motion = CircularMotion { center_x: 0.0, center_y: 0.0, radius: 5.0 };
        motion.calculate_positions("CW"); // This calls the method with the "CW" direction
        // Assert results if necessary
    }

    #[test]
    fn test_circular_motion_positions_ccw() {
        let motion = CircularMotion { center_x: 0.0, center_y: 0.0, radius: 5.0 };
        motion.calculate_positions("CCW"); // This calls the method with the "CCW" direction
        // Assert results if necessary
    }
}
