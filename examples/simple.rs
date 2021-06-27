use fgui::prelude::*;

#[derive(Debug)]
pub struct App {
    should_render: bool,
    offset: f32,
}

impl fgui::App for App {}

impl App {
    fn new() -> App {
        App {
            should_render: true,
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
        todo!()
    }

    fn render(&self, mut surface: Area) {
        use fgui::graphics::Vertex;

        surface.backend.add_vertex(Vertex {
            position: [50.0 + self.offset, 100.0],
            color: [1.0, 0.0, 0.0, 1.0],
        });
        surface.backend.add_vertex(Vertex {
            position: [0.0 + self.offset, 0.0],
            color: [0.0, 1.0, 0.0, 1.0],
        });
        surface.backend.add_vertex(Vertex {
            position: [100.0 + self.offset, 0.0],
            color: [0.0, 0.0, 1.0, 1.0],
        });
    }
}

fn main() {
    env_logger::init();
    run(App::new())
}
