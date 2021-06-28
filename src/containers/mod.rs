pub mod flexbox;

pub use flexbox::Flexbox;

/// This defines the alignment along the main axis.
///
/// It helps distribute extra free space leftover when either all the flex items on a line are inflexible, or are flexible but have reached their maximum size.
/// It also exerts some control over the alignment of items when they overflow the line.
///
/// <img src="https://css-tricks.com/wp-content/uploads/2018/10/justify-content.svg" alt="flex items within a flex container demonstrating the different spacing options" width="50%"/>
#[derive(Debug)]
pub enum JustifyContent {
    /// Items are packed toward the left. (default)
    Left,
    /// Items are packed toward the right.
    Right,
    /// Items are centered along the line
    Center,
    /// Items are evenly distributed in the line; first item is on the start line, last item on the end line
    SpaceBetween,
    /// Items are evenly distributed in the line with equal space around them.
    /// Note that visually the spaces aren’t equal, since all the items have equal space on both sides.
    /// The first item will have one unit of space against the container edge, but two units of space between the next item because that next item has its own spacing that applies.
    SpaceAround,
    /// Items are distributed so that the spacing between any two items (and the space to the edges) is equal.
    SpaceEvenly,
    // Todo support other values
}

/// By default, flex items will all try to fit onto one line.
/// You can change that and allow the items to wrap as needed with this property.
#[derive(Debug)]
pub enum FlexWrap {
    /// All flex items will be on one line. (default)
    NoWrap,
    /// FLex items will wrap onto multiple lines, from top to bottom.
    Wrap,
    /// Flex items will wrap onto multiple lines from bottom to top.
    WrapReverse,
}

/// This defines the default behavior for how flex items are laid out along the cross axis on the current line.
///
/// Think of it as the [`JustifyContent`] version for the cross-axis (perpendicular to the main-axis).
///
/// <img src="https://css-tricks.com/wp-content/uploads/2018/10/align-items.svg" alt="demonstration of differnet alignment options, like all boxes stuck to the top of a flex parent, the bottom, stretched out, or along a baseline" width="50%"/>
///
/// Baseline is not supported yet.
#[derive(Debug)]
pub enum AlignItems {
    /// Stretch to fill the container (still respect min-width/max-width). (default)
    Stretch,
    /// Items are placed at the top.
    Top,
    /// Items are placed at the bottom.
    Bottom,
    /// Items are centered in the cross-axis.
    Center,
}

/// This aligns a flex container’s lines within when there is extra space in the cross-axis, similar to how [`JustifyContent`] aligns individual items within the main-axis.
///
/// Note: This property only takes effect on multi-line flexible containers, where [`FlexWrap`] is set to either [`FlexWrap::Wrap`] or [`FlexWrap::WrapReverse`]). A single-line flexible container (i.e. where [`FlexWrap`] is set to its default value, [`FlexWrap::NoWrap`]) will not reflect [`AlignContent`].
///
/// <img src="https://css-tricks.com/wp-content/uploads/2018/10/align-content.svg" alt="examples of the align-content property where a group of items cluster at the top or bottom, or stretch out to fill the space, or have spacing." width="50%"/>
#[derive(Debug)]
pub enum AlignContent {
    /// Items packed to the top of the container (default).
    Top,
    /// Items packed to the bottom of the container.
    Bottom,
    /// Items centered in the container.
    Center,
    /// Items evenly distributed; the first line is at the start of the container while the last one is at the end.
    SpaceBetween,
    /// Items evenly distributed with equal space around each line.
    SpaceAround,
    /// Items are evenly distributed with equal space around them.
    SpaceEvenly,
    /// Lines stretch to take up the remaining space.
    Stretch,
}
