use fgui::containers::*;
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

pub struct BigRectangle {}

impl Widget for BigRectangle {
    fn allocate_area(
        &mut self,
        _screen_size: (usize, usize),
        _container_size: (usize, usize),
    ) -> WidgetSize {
        WidgetSize {
            min_width: 60.0,
            width: 80.0,
            max_width: 210.0,
            min_height: 30.0,
            height: 35.0,
            max_height: 60.0,
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
    div: fgui::containers::Div,
    offset: f32,
}

impl fgui::App for App {}

impl App {
    fn new() -> App {
        let mut div = Div::new();
        div.add_block(Box::new(BigRectangle {}));
        div.add_block(Box::new(BigRectangle {}));
        div.add_inline(Box::new(Rectangle {}));
        div.add_inline(Box::new(Rectangle {}));
        div.add_inline(Box::new(Rectangle {}));

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
