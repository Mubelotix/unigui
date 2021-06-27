use crate::*;

#[derive(Debug, PartialEq)]
pub struct WidgetSize {
    pub min_width: f32,
    pub width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub height: f32,
    pub max_height: f32,
}

impl WidgetSize {
    pub fn set_size(&mut self, width: f32, height: f32) {
        self.set_width(width);
        self.set_height(height);
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width.clamp(self.min_width, self.max_width);
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height.clamp(self.min_height, self.max_height);
    }

    pub fn fit_height(&mut self, restrictions: &WidgetSize) {
        let height = self
            .height
            .clamp(restrictions.min_height, restrictions.max_height);
        self.height = height.clamp(self.min_height, self.max_height);
    }

    pub fn fit_width(&mut self, restrictions: &WidgetSize) {
        let width = self
            .width
            .clamp(restrictions.min_width, restrictions.max_width);
        self.width = width.clamp(self.min_width, self.max_width);
    }
}

pub trait Widget {
    /// Allows the widget to update its internal state.
    /// Widgets should use message passing to collect events.
    fn update(&mut self) {}

    /// This function allows the widget to choose its size.  
    /// Since arguments are passed, the widget can be responsive.  
    /// This function will be called at every frame, before [Widget::render].  
    ///   
    /// The container of the widget will then choose the final size and placement of the widget.  
    /// [Widget::render] will always be called after this.
    fn allocate_area(
        &mut self,
        screen_size: (usize, usize),
        container_size: (usize, usize),
    ) -> WidgetSize;

    /// This function will always be called after [Widget::allocate_area].  
    /// The widget should consider that it owns the passed [Area] of the screen.
    fn render(&self, area: Area);
}
