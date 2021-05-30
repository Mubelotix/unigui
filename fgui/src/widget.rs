use crate::*;

#[derive(Debug, PartialEq)]
pub struct WidgetSize {
    pub min_width: usize,
    pub width: usize,
    pub max_width: usize,
    pub min_height: usize,
    pub height: usize,
    pub max_height: usize,
}

impl WidgetSize {
    pub fn set_size(&mut self, width: usize, height: usize) {
        self.width = width.clamp(self.min_width, self.max_width);
        self.height = height.clamp(self.min_height, self.max_height);
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height.clamp(self.min_height, self.max_height);
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width.clamp(self.min_width, self.max_width);
    }
}

pub trait Widget<Backend: BackendTrait> {
    /// Allows the widget to update its internal state.
    fn update(&mut self) {}

    /// This function will be called at each frame to ask if render is required.
    /// If the widget returns `true`, then it will be rendered. The next function to be called will be [Widget::allocate_area].  
    /// If the widget returns `false`, rendering of this object will be cancelled.
    fn accept_render(&mut self) -> bool; // Todo: they should not modify their state here as it may not be called

    /// This function will usually be called after [Widget::accept_render], but may be called independently (on window resize for example).
    /// 
    /// It allows the widget to choose its size.  
    /// Since arguments are passed, the widget can be responsive. 
    /// 
    /// The container of the widget will then choose the final size and placement of the widget.  
    /// [Widget::render] will always be called after this.
    fn allocate_area(&mut self, screen_size: (usize, usize), container_size: (usize, usize)) -> WidgetSize;

    /// This function will always be called after [Widget::allocate_area].  
    /// The widget should consider that it owns the passed [Area] of the screen.
    fn render(&self, area: Area<Backend>);
}
