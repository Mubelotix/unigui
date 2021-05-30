use crate::*;

pub struct Area<'a, Backend: BackendTrait> {
    pub rect: Rect, // TODO visibility
    pub backend: &'a mut Backend,
}

impl<'a, Backend: BackendTrait> Area<'a, Backend> {
    pub fn new(rect: Rect, backend: &'a mut Backend) -> Area<'a, Backend> {
        Area {
            rect,
            backend,
        }
    }

    pub fn width(&self) -> isize {
        self.rect.bottom_right.0 - self.rect.top_left.0
    }

    pub fn height(&self) -> isize {
        self.rect.bottom_right.1 - self.rect.top_left.1
    }

    pub fn subarea(&'a mut self, rect: Rect) -> Area<'a, Backend> {
        Area {
            rect: self.rect.to_owned() + rect,
            backend: self.backend
        }
    }
}
