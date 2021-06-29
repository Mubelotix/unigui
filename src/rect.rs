#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Rect {
    pub top_left: (f32, f32),
    pub bottom_right: (f32, f32),
}

impl Rect {
    pub fn sized(x: f32, y: f32, width: f32, height: f32) -> Rect {
        // check width (must be positive)
        Rect {
            top_left: (x, y),
            bottom_right: (x + width, y + height),
        }
    }

    pub fn width(&self) -> f32 {
        self.bottom_right.0 - self.top_left.0
    }

    pub fn height(&self) -> f32 {
        self.bottom_right.1 - self.top_left.1
    }
}
