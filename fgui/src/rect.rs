#[derive(Debug, Clone)]
pub struct Rect {
    pub top_left: (isize, isize),
    pub bottom_right: (isize, isize),
}

impl Rect {
    pub fn sized(x: isize, y: isize, width: usize, height: usize) -> Rect {
        Rect {
            top_left: (x, y),
            bottom_right: (x+width as isize, y+height as isize),
        }
    }
}
