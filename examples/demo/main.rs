use fgui::containers::*;
use fgui::prelude::*;
use std::cell::RefCell;

#[path = "unigui-classic/src/lib.rs"]
mod unigui_classic;
use unigui_classic::button::Button;

#[derive(Debug)]
pub struct App {
    image_id: RefCell<Option<TextureId>>,
    div: fgui::containers::Div,
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
            image_id: RefCell::new(None),
            div,
        }
    }
}

impl fgui::Widget for App {
    fn update(&mut self) {}

    fn allocate_area(
        &mut self,
        screen_size: (usize, usize),
        container_size: (usize, usize),
    ) -> WidgetSize {
        self.div.allocate_area(screen_size, container_size)
    }

    fn render(&self, surface: Area) {
        if self.image_id.borrow().is_none() {
            let texture_bytes = include_bytes!("happy-tree.png");
            let texture_image = image::load_from_memory(texture_bytes).unwrap();
            let texture_rgba = texture_image.as_rgba8().unwrap();

            use image::GenericImageView;
            let dimensions = texture_image.dimensions();
            let texture_id = surface.backend.create_texture(dimensions, &texture_rgba);
            *self.image_id.borrow_mut() = Some(texture_id);
        }

        if let Some(texture_id) = self.image_id.borrow().clone() {
            surface
                .backend
                .add_image(Rect::sized(500.0, 0.0, 100.0, 100.0), texture_id)
        }

        surface
            .backend
            .add_text(glyph_brush::Section::default().add_text(
                glyph_brush::Text::new("The last word was ").with_color([0.0, 0.0, 1.0, 1.0]),
            ));

        self.div.render(surface);
    }
}

fn main() {
    env_logger::init();
    run(App::new())
}
