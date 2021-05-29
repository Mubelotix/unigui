use crate::*;

pub enum WidgetSize {
    /// This size **will** be the final size of the widget.
    /// The container will not be able to resize the widget in any way.
    Fixed { width: usize, height: usize },
    /// The size may be infinitely larger but will probably be the specified size.
    Minimum { width: usize, height: usize },
    /// The size may be infinitely smaller but will probably be the specified size.
    Maximum { width: usize, height: usize },
    /// The size will probably be the default, but may be resized by the container of the widget in the respect of the requirements.
    Range { default_width: usize, default_height: usize, width: std::ops::Range<usize>, height: std::ops::Range<usize> },
}

pub trait Widget<Backend: BackendTrait> {
    type Message;

    fn create(link: Link<Self::Message>) -> Self;

    /// Allows the widget to update its internal state.
    fn update(&mut self, _msg: Self::Message);

    /// This function will be called at each frame to ask if render is required.
    /// If the widget returns `true`, then it will be rendered. The next function to be called will be [Widget::allocate_area].  
    /// If the widget returns `false`, rendering of this object will be cancelled.
    fn accept_render(&self) -> bool;

    /// This function will usually be called after [Widget::accept_render], but may be called independently (on window resize for example).
    /// 
    /// It allows the widget to choose its size.  
    /// Since arguments are passed, the widget can be responsive. 
    /// 
    /// The container of the widget will then choose the final size and placement of the widget.  
    /// [Widget::render] will always be called after this.
    fn allocate_area(&self) -> WidgetSize;

    /// This function will always be called after [Widget::allocate_area].  
    /// The widget should consider that it owns the passed [Area] of the screen.
    fn render(&self, area: Area<Backend>);
}
