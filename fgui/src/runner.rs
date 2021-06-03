use crate::*;

pub fn run<Backend: BackendTrait, App: crate::app::App<Backend>>(app: App) {
    Backend::run(app)
}
