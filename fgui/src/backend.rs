use crate::*;

pub trait Backend: Sized {
    fn run(app: impl App<Self>) -> !;
}
