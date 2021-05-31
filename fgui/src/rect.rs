#[derive(Debug, Clone, PartialEq, Copy)]
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

impl std::ops::Add<Rect> for Rect {
    type Output = Rect;

    fn add(mut self, rhs: Rect) -> Self::Output {
        self.top_left.0 += rhs.top_left.0;
        self.top_left.1 += rhs.top_left.1;
        self.bottom_right.0 += rhs.bottom_right.0;
        self.bottom_right.1 += rhs.bottom_right.1;
        self
    }
}
