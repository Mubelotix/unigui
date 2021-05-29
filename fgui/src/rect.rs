#[derive(Debug, Clone)]
pub struct Rect {
    pub top_left: (usize, usize),
    pub bottom_right: (usize, usize),
}

impl Rect {
    pub fn sized(x: usize, y: usize, width: usize, height: usize) -> Rect {
        Rect {
            top_left: (x, y),
            bottom_right: (width, height),
        }
    }
}
