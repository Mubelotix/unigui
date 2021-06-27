use crate::*;

pub struct Area<'a> {
    pub rect: Rect, // TODO visibility
    pub backend: &'a mut WgpuBackend,
}

impl<'a> Area<'a> {
    pub fn new(rect: Rect, backend: &'a mut WgpuBackend) -> Area<'a> {
        Area {
            rect,
            backend,
        }
    }

    pub fn width(&self) -> f32 {
        self.rect.bottom_right.0 - self.rect.top_left.0
    }

    pub fn height(&self) -> f32 {
        self.rect.bottom_right.1 - self.rect.top_left.1
    }

    pub fn subarea(&mut self, rect: Rect) -> Area {
        Area {
            rect: self.rect + rect,
            backend: self.backend
        }
    }
}
