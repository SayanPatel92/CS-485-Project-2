use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseFloatError;

// Error conversion function for better error handling
fn parse_float_error_to_io_error(e: ParseFloatError) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, e.to_string())
}

// Structs and implementations for motion types
struct LinearMotion {
    start_x: f64,
    start_y: f64,
    start_z: f64,
    end_x: f64,
    end_y: f64,
    end_z: f64,
}

impl LinearMotion {
    fn calculate_positions(&self) {
        let steps = ((self.end_x - self.start_x).hypot(self.end_y - self.start_y).hypot(self.end_z - self.start_z)).ceil() as usize;
        for i in 1..=steps {
            let t = i as f64 / steps as f64;
            let x = self.start_x + t * (self.end_x - self.start_x);
            let y = self.start_y + t * (self.end_y - self.start_y);
            let z = self.start_z + t * (self.end_z - self.start_z);
            println!("{:.2}, {:.2}, {:.2}", x, y, z);
        }
    }
}

struct CircularMotion {
    center_x: f64,
    center_y: f64,
    center_z: f64,
    radius: f64,
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

// Main logic to read commands from a file and process them
fn main() -> io::Result<()> {
    let file_path = "C:\\Users\\patel\\OneDrive\\Attachments\\CSCI 485 Programming Lang\\Project 2\\commands.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut start_x = 0.0;
    let mut start_y = 0.0;
    let mut start_z = 0.0;

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "LIN" => {
                let end_x = parts[1].strip_prefix("X").unwrap_or("0").parse::<f64>().map_err(parse_float_error_to_io_error)?;
                let end_y = parts[2].strip_prefix("Y").unwrap_or("0").parse::<f64>().map_err(parse_float_error_to_io_error)?;
                let end_z = parts[3].strip_prefix("Z").unwrap_or("0").parse::<f64>().map_err(parse_float_error_to_io_error)?;
                let motion = LinearMotion { start_x, start_y, start_z, end_x, end_y, end_z };
                motion.calculate_positions();
                start_x = end_x;
                start_y = end_y;
                start_z = end_z;
            },
            "CW" | "CCW" => {
                let center_x = parts[1].strip_prefix("X").unwrap_or("0").parse::<f64>().map_err(parse_float_error_to_io_error)?;
                let center_y = parts[2].strip_prefix("Y").unwrap_or("0").parse::<f64>().map_err(parse_float_error_to_io_error)?;
                let center_z = parts[3].strip_prefix("Z").unwrap_or("0").parse::<f64>().map_err(parse_float_error_to_io_error)?; // Ensure you have Z part in your input data
                let radius = parts[4].strip_prefix("I").unwrap_or("0").parse::<f64>().map_err(parse_float_error_to_io_error)?;
                let motion = CircularMotion { center_x, center_y, center_z, radius };
                motion.calculate_positions(parts[0]);  // parts[0] should be "CW" or "CCW"
            },
            _ => eprintln!("Unknown command type: {}", parts[0]),
        }
    }

    Ok(())
}