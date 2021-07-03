use unigui::containers::*;
use unigui::prelude::*;

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
        use unigui::graphics::Vertex;

        surface.backend.add_vertex(Vertex {
            position: [surface.rect.min.0 + 2.0, surface.rect.min.1 + 2.0],
            color: [1.0, 0.0, 0.0, 1.0],
        });
        surface.backend.add_vertex(Vertex {
            position: [surface.rect.max.0 - 2.0, surface.rect.min.1 + 2.0],
            color: [1.0, 0.0, 0.0, 1.0],
        });
        surface.backend.add_vertex(Vertex {
            position: [surface.rect.min.0 + 2.0, surface.rect.max.1 - 2.0],
            color: [1.0, 0.0, 0.0, 1.0],
        });

        surface.backend.add_vertex(Vertex {
            position: [surface.rect.min.0 + 2.0, surface.rect.max.1 - 2.0],
            color: [1.0, 0.0, 0.0, 1.0],
        });
        surface.backend.add_vertex(Vertex {
            position: [surface.rect.max.0 - 2.0, surface.rect.min.1 + 2.0],
            color: [1.0, 0.0, 0.0, 1.0],
        });
        surface.backend.add_vertex(Vertex {
            position: [surface.rect.max.0 - 2.0, surface.rect.max.1 - 2.0],
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
        use unigui::graphics::Vertex;

        surface.backend.add_vertex(Vertex {
            position: [surface.rect.min.0 + 2.0, surface.rect.min.1 + 2.0],
            color: [1.0, 0.0, 0.0, 1.0],
        });
        surface.backend.add_vertex(Vertex {
            position: [surface.rect.max.0 - 2.0, surface.rect.min.1 + 2.0],
            color: [1.0, 0.0, 0.0, 1.0],
        });
        surface.backend.add_vertex(Vertex {
            position: [surface.rect.min.0 + 2.0, surface.rect.max.1 - 2.0],
            color: [1.0, 0.0, 0.0, 1.0],
        });

        surface.backend.add_vertex(Vertex {
            position: [surface.rect.min.0 + 2.0, surface.rect.max.1 - 2.0],
            color: [1.0, 0.0, 0.0, 1.0],
        });
        surface.backend.add_vertex(Vertex {
            position: [surface.rect.max.0 - 2.0, surface.rect.min.1 + 2.0],
            color: [1.0, 0.0, 0.0, 1.0],
        });
        surface.backend.add_vertex(Vertex {
            position: [surface.rect.max.0 - 2.0, surface.rect.max.1 - 2.0],
            color: [1.0, 0.0, 0.0, 1.0],
        });
    }
}

#[derive(Debug)]
pub struct App {
    should_render: bool,
    flexbox: unigui::containers::Flexbox,
    offset: f32,
}

impl unigui::App for App {}

impl App {
    fn new() -> App {
        let mut flexbox = Flexbox::new();
        flexbox.set_area_allocator(Some(Box::new(|_screen_size, container_size| WidgetSize {
            min_width: 0.0,
            width: 0.0,
            max_width: container_size.0 as f32,
            min_height: 200.0,
            height: 200.0,
            max_height: container_size.1 as f32,
        })));
        flexbox.set_flex_wrap(FlexWrap::Wrap);
        flexbox.set_justify_content(JustifyContent::SpaceEvenly);
        flexbox.set_align_content(AlignContent::SpaceEvenly);
        flexbox.set_align_items(AlignItems::Center);
        flexbox.add(Box::new(Rectangle {}));
        flexbox.add(Box::new(BigRectangle {}));
        flexbox.add(Box::new(Rectangle {}));
        flexbox.add(Box::new(BigRectangle {}));
        flexbox.add(Box::new(Rectangle {}));

        App {
            should_render: true,
            flexbox,
            offset: 0.0,
        }
    }
}

impl unigui::Widget for App {
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

#[test]
fn test() {
    main()
}
