use crate::{*, link::create_link};

pub fn run<Backend: BackendTrait, App: crate::app::App<Backend>>() {
    let mut backend = Backend::init();
    let (link, link_receiver) = create_link();
    let mut app = App::create(link);

    let surface = backend.clear();
    app.render(surface);

    loop {
        let instant = std::time::Instant::now() + std::time::Duration::from_millis(16);
        while let Ok(message) = link_receiver.receiver.recv_timeout(instant - std::time::Instant::now()) {
            app.update(message);
        }

        if app.accept_render() {
            let surface = backend.clear();
            app.render(surface);
        }
    }
}
