use crate::*;

pub fn run<App: crate::app::App + 'static>(app: App) {
    WgpuBackend::run(app)
}
