use fgui::containers::{FlexWrap, Flexbox};
use fgui::prelude::*;

pub struct Rectangle {}

impl Widget for Rectangle {
    fn allocate_area(
        &mut self,
        _screen_size: (usize, usize),
        _container_size: (usize, usize),
    ) -> WidgetSize {
        WidgetSize {
            min_width: 50.0,
            width: 75.0,
            max_width: 200.0,
            min_height: 20.0,
            height: 25.0,
            max_height: 50.0,
        }
    }

    fn render(&self, surface: Area) {
        use fgui::graphics::Vertex;

        surface.backend.add_vertex(Vertex {
            position: [surface.rect.top_left.0 + 2.0, surface.rect.top_left.1 + 2.0],
            color: [1.0, 0.0, 0.0, 1.0],
        });
        surface.backend.add_vertex(Vertex {
            position: [
                surface.rect.bottom_right.0 - 2.0,
                surface.rect.top_left.1 + 2.0,
            ],
            color: [1.0, 0.0, 0.0, 1.0],
        });
        surface.backend.add_vertex(Vertex {
            position: [
                surface.rect.top_left.0 + 2.0,
                surface.rect.bottom_right.1 - 2.0,
            ],
            color: [1.0, 0.0, 0.0, 1.0],
        });

        surface.backend.add_vertex(Vertex {
            position: [
                surface.rect.top_left.0 + 2.0,
                surface.rect.bottom_right.1 - 2.0,
            ],
            color: [1.0, 0.0, 0.0, 1.0],
        });
        surface.backend.add_vertex(Vertex {
            position: [
                surface.rect.bottom_right.0 - 2.0,
                surface.rect.top_left.1 + 2.0,
            ],
            color: [1.0, 0.0, 0.0, 1.0],
        });
        surface.backend.add_vertex(Vertex {
            position: [
                surface.rect.bottom_right.0 - 2.0,
                surface.rect.bottom_right.1 - 2.0,
            ],
            color: [1.0, 0.0, 0.0, 1.0],
        });
    }
}

#[derive(Debug)]
pub struct App {
    should_render: bool,
    flexbox: fgui::containers::Flexbox,
    offset: f32,
}

impl fgui::App for App {}

impl App {
    fn new() -> App {
        let mut flexbox = Flexbox::new();
        flexbox.set_flex_wrap(FlexWrap::Wrap);
        flexbox.add(Box::new(Rectangle {}));
        flexbox.add(Box::new(Rectangle {}));
        flexbox.add(Box::new(Rectangle {}));

        App {
            should_render: true,
            flexbox,
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
        self.flexbox.allocate_area(screen_size, container_size)
    }

    fn render(&self, surface: Area) {
        self.flexbox.render(surface);
    }
}

fn main() {
    env_logger::init();
    run(App::new())
}
