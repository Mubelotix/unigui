use crate::*;

pub fn run<App: crate::app::App>(app: App) {
    WgpuBackend::run(app)
}
