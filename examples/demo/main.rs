use fgui::containers::*;
use fgui::prelude::*;

#[path = "unigui-classic/src/lib.rs"]
mod unigui_classic;
use unigui_classic::button::Button;

#[derive(Debug)]
pub struct App {
    should_render: bool,
    div: fgui::containers::Div,
    offset: f32,
}

impl fgui::App for App {}

impl App {
    fn new() -> App {
        let mut div = Div::new();
        div.add_block(Box::new(Button {}));
        div.add_block(Box::new(Button {}));
        div.add_inline(Box::new(Button {}));
        div.add_inline(Box::new(Button {}));
        div.add_inline(Box::new(Button {}));

        App {
            should_render: true,
            div,
            offset: 0.0,
        }
    }
}

impl fgui::Widget for App {
    fn update(&mut self) {
        self.offset += 0.01;
    }

    fn allocate_area(
        &mut self,
        screen_size: (usize, usize),
        container_size: (usize, usize),
    ) -> WidgetSize {
        self.div.allocate_area(screen_size, container_size)
    }

    fn render(&self, surface: Area) {
        self.div.render(surface);
    }
}

fn main() {
    env_logger::init();
    run(App::new())
}
