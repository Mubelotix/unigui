use crate::*;

pub struct Area<'a, Backend: BackendTrait> {
    pub area: Rect, // TODO visibility
    pub backend: &'a mut Backend,
}

impl<'a, Backend: BackendTrait> Area<'a, Backend> {
    pub fn new(area: Rect, backend: &'a mut Backend) -> Area<'a, Backend> {
        Area {
            area,
            backend,
        }
    }
}
