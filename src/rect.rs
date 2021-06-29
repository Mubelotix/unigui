#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Rect {
    pub min: (f32, f32),
    pub max: (f32, f32),
}

impl Rect {
    pub fn sized(x: f32, y: f32, width: f32, height: f32) -> Rect {
        // check width (must be positive)
        Rect {
            min: (x, y),
            max: (x + width, y + height),
        }
    }

    pub fn width(&self) -> f32 {
        self.max.0 - self.min.0
    }

    pub fn height(&self) -> f32 {
        self.max.1 - self.min.1
    }
}
