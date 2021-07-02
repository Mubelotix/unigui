use fgui::prelude::*;
use lyon::path::traits::PathBuilder;

pub struct Button {}

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
        use lyon::math::{point, size, Point};
        use lyon::path::{builder::BorderRadii, Path, Winding};
        use lyon::tessellation::{geometry_builder::simple_builder, *};

        // Create a simple path.
        let mut path_builder = Path::builder();
        path_builder.add_rounded_rectangle(
            &lyon::math::Rect::new(
                point(surface.rect.min.0 + 2.5, surface.rect.min.1 + 2.5),
                size(surface.rect.width() - 5.0, surface.rect.height() - 5.0),
            ),
            &BorderRadii::new(5.0),
            Winding::Negative,
        );
        let path = path_builder.build();

        // Create the destination vertex and index buffers.
        let mut buffers: VertexBuffers<Point, u16> = VertexBuffers::new();

        {
            let mut vertex_builder = simple_builder(&mut buffers);

            // Create the tessellator.
            let mut tessellator = FillTessellator::new();

            // Compute the tessellation.
            let result =
                tessellator.tessellate_path(&path, &FillOptions::default(), &mut vertex_builder);
            assert!(result.is_ok());
        }

        for i in buffers.indices {
            let vertex = buffers.vertices[i as usize];
            let vertex = Vertex {
                position: [vertex.x, vertex.y],
                color: [1.0, 0.0, 0.0, 1.0],
            };
            surface.backend.add_vertex(vertex);
        }

        surface.backend.add_text(
            glyph_brush::Section::default()
                .with_screen_position((surface.rect.min.0 + 8.0, surface.rect.min.1 + 4.0))
                .add_text(glyph_brush::Text::new("Button!").with_color([1.0, 1.0, 1.0, 1.0])),
        );
    }
}
