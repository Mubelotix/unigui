use fgui::prelude::*;

pub struct Button {

}

impl Widget for Button {
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
