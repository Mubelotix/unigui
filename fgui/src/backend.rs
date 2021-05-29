use crate::*;

pub trait Backend: Sized {
    fn init() -> Self;

    fn clear(&mut self) -> Area<Self>;
}
