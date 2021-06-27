use fgui::prelude::*;

#[derive(Debug)]
pub struct App {
    should_render: bool,
}

impl fgui::App for App {}

impl App {
    fn new() -> App {
        App {
            should_render: true,
        }
    }
}

impl fgui::Widget for App {
    fn update(&mut self) {}

    fn accept_render(&self) -> bool {
        self.should_render
    }

    fn allocate_area(
        &mut self,
        screen_size: (usize, usize),
        container_size: (usize, usize),
    ) -> WidgetSize {
        todo!()
    }

    fn render(&self, mut surface: Area) {}
}

fn main() {
    env_logger::init();
    run(App::new())
}
