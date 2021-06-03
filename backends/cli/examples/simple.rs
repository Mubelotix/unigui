use fgui::*;
use fgui_cli::{CliArea, CliBackend};

#[derive(Debug)]
pub struct App {
    link: Link<()>,
    should_render: bool,
}

impl fgui::App<CliBackend> for App {}

impl fgui::Widget<CliBackend> for App {
    type Message = ();

    fn create(link: fgui::Link<Self::Message>) -> Self {
        App {
            link,
            should_render: true,
        }
    }

    fn render(&self, mut surface: Area<CliBackend>) {
        surface.print("message")
    }

    fn update(&mut self, _msg: Self::Message) {
        
    }

    fn accept_render(&self) -> bool {
        self.should_render
    }

    fn allocate_area(&self) -> widget::WidgetSize {
        todo!()
    }
}

fn main() {
    run::<CliBackend, App>()
}
