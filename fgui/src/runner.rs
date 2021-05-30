use crate::*;

pub fn run<Backend: BackendTrait, App: crate::app::App<Backend>>(mut app: App) {
    let mut backend = Backend::init();

    let surface = backend.clear();
    app.render(surface);

    loop {
        if app.accept_render() {
            let surface = backend.clear();
            app.render(surface);
        }
    }
}
