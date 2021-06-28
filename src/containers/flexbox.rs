use crate::*;
use containers::*;

/// A function that takes the screen size and the container size, returning the required size of an item.
type AreaAllocator = Box<dyn FnMut((usize, usize), (usize, usize)) -> WidgetSize>;

fn max(i1: f32, i2: f32) -> f32 {
    if i1 > i2 {
        i1
    } else {
        i2
    }
}

fn min(i1: f32, i2: f32) -> f32 {
    if i1 < i2 {
        i1
    } else {
        i2
    }
}

pub struct Flexbox {
    widgets: Vec<Box<dyn Widget>>,
    widget_subareas: Vec<Rect>,
    align_content: AlignContent,
    align_items: AlignItems,
    flex_wrap: FlexWrap,
    justify_content: JustifyContent,
    area_allocator: Option<AreaAllocator>,
    must_render: bool,
}

impl Widget for Flexbox {
    fn allocate_area(
        &mut self,
        screen: (usize, usize),
        container_size: (usize, usize),
    ) -> WidgetSize {
        // Get the size restrictions of the flexbox
        let mut container = match &mut self.area_allocator {
            Some(area_allocator) => area_allocator(screen, container_size),
            None => WidgetSize {
                min_width: 0.0,
                width: 0.0,
                max_width: container_size.0 as f32,
                min_height: 0.0,
                height: 0.0,
                max_height: container_size.1 as f32,
            },
        };

        // Get the size of the widgets and wrap them if needed
        let mut rows = Vec::new();
        match self.flex_wrap {
            FlexWrap::NoWrap => {
                let (mut row, mut row_width, mut row_height) = (Vec::new(), 0.0, 0.0);
                for widget in &mut self.widgets {
                    let widget_size = widget.allocate_area(
                        screen,
                        (container.max_width as usize, container.max_height as usize),
                    );
                    row_width += widget_size.width;
                    row_height = max(row_height, widget_size.height);
                    row.push(widget_size);
                }
                rows.push((row, row_width, row_height));
            }
            FlexWrap::Wrap => {
                let (mut row, mut row_width, mut row_height) = (Vec::new(), 0.0, 0.0);
                for widget in &mut self.widgets {
                    let widget_size = widget.allocate_area(
                        screen,
                        (container.max_width as usize, container.max_height as usize),
                    );

                    if row_width + widget_size.width > container.max_width {
                        rows.push((row, row_width, row_height));
                        row_width = 0.0;
                        row_height = 0.0;
                        row = Vec::new();
                    }

                    row_width += widget_size.width;
                    row_height = max(row_height, widget_size.height);
                    row.push(widget_size);
                }
                rows.push((row, row_width, row_height));
            }
            FlexWrap::WrapReverse => {
                todo!()
            }
        }

        // Resize to fix horizontal overflows
        // Each row will independently try to fit in the space
        for (widgets, row_width, _) in &mut rows {
            if *row_width > container.max_width {
                let mut overflow = *row_width - container.max_width;
                let mut total_shrink_capacity = 0.0;
                for widget in widgets.iter() {
                    total_shrink_capacity += widget.width - widget.min_width;
                }

                // Shrink the sizes as much as needed
                *row_width = 0.0;
                for widget in widgets.iter_mut() {
                    let shrink_capacity = widget.width - widget.min_width;
                    let factor = (overflow / total_shrink_capacity).clamp(0.0, 1.0);
                    let to_skrink = shrink_capacity * factor;

                    overflow -= to_skrink;
                    widget.width -= to_skrink;
                    total_shrink_capacity -= shrink_capacity;
                    *row_width += widget.width;
                }
            }
        }

        // Resize to fix vertical overflows
        // The rows will try to fit in the space
        let mut flexbox_height = 0.0;
        for (_, _, row_height) in &rows {
            flexbox_height += row_height;
        }
        if flexbox_height > container.max_height {
            // Calculate the overflow and calculate the shrinkable size
            let mut overflow = flexbox_height - container.max_height;
            let mut shrinkables = Vec::new();
            for (row, _, row_height) in &mut rows {
                let mut min_row_height = 0.0;
                for widget in row.iter() {
                    min_row_height = max(min_row_height, widget.min_height);
                }
                shrinkables.push((*row_height - min_row_height, row, row_height));
            }
            shrinkables.sort_by_key(|(amount, _, _)| (*amount * 1000.0) as isize);

            // Shrink the sizes as much as possible until the size fits
            let mut remaining_blocks = shrinkables.len();
            for (mut amount, row, row_height) in shrinkables {
                let to_skrink = overflow / remaining_blocks as f32;
                amount = min(amount, to_skrink);
                for widget in row {
                    widget.height -= amount;
                }
                *row_height -= amount;
                overflow -= amount;

                remaining_blocks -= 1;
            }
        }

        // Get final size
        let mut flexbox_width = 0.0;
        flexbox_height = 0.0;
        for (_, row_width, row_height) in &rows {
            flexbox_width = max(flexbox_width, *row_width);
            flexbox_height += row_height;
        }
        container.set_size(flexbox_width, flexbox_height);

        // Prepare subareas
        let mut x;
        let (mut y, y_offset) = match self.align_content {
            AlignContent::FlexStart => (0.0, 0.0),
            AlignContent::FlexEnd => {
                let mut y = container.height - flexbox_height;
                if y < 0.0 {
                    y = 0.0;
                }
                (y, 0.0)
            }
            AlignContent::Center => {
                let mut y = (container.height - flexbox_height) / 2.0;
                if y < 0.0 {
                    y = 0.0;
                }
                (y, 0.0)
            }
            AlignContent::SpaceBetween if rows.len() <= 1 => {
                let mut y = (container.height - flexbox_height) / 2.0;
                if y < 0.0 {
                    y = 0.0;
                }
                (y, 0.0)
            }
            AlignContent::SpaceBetween => {
                let mut vertical_space_between_amount =
                    (container.height - flexbox_height) / (rows.len() - 1) as f32;
                if vertical_space_between_amount < 0.0 {
                    vertical_space_between_amount = 0.0;
                }
                (0.0, vertical_space_between_amount)
            }
            AlignContent::SpaceAround => {
                let mut vertical_space_between_amount =
                    (container.height - flexbox_height) / rows.len() as f32;
                if vertical_space_between_amount < 0.0 {
                    vertical_space_between_amount = 0.0;
                }
                (
                    vertical_space_between_amount / 2.0,
                    vertical_space_between_amount,
                )
            }
            AlignContent::SpaceEvenly => {
                let mut vertical_space_between_amount =
                    (container.height - flexbox_height) / (rows.len() + 1) as f32;
                if vertical_space_between_amount < 0.0 {
                    vertical_space_between_amount = 0.0;
                }
                (vertical_space_between_amount, vertical_space_between_amount)
            }
            AlignContent::Stretch if rows.is_empty() => (0.0, 0.0),
            AlignContent::Stretch => {
                let mut excess_height = container.height - flexbox_height;
                if excess_height < 0.0 {
                    excess_height = 0.0;
                }

                let height_to_add = excess_height / rows.len() as f32;
                for (widgets, _row_width, row_height) in &mut rows {
                    *row_height += height_to_add;
                    for widget in widgets {
                        widget.set_height(*row_height);
                    }
                }

                (0.0, 0.0)
            }
        };
        self.widget_subareas.clear();
        for (row, row_width, row_height) in &rows {
            let (new_x, x_offset) = match self.justify_content {
                JustifyContent::FlexStart => (0.0, 0.0),
                JustifyContent::FlexEnd => (container_size.0 as f32 - row_width, 0.0),
                JustifyContent::Center => ((container_size.0 as f32 - row_width) / 2.0, 0.0),
                JustifyContent::SpaceBetween if row.len() <= 1 => {
                    ((container_size.0 as f32 - row_width) / 2.0, 0.0)
                }
                JustifyContent::SpaceBetween => {
                    let mut space_between_amount =
                        (container_size.0 as f32 - row_width) / (row.len() - 1) as f32;
                    if space_between_amount < 0.0 {
                        space_between_amount = 0.0;
                    }
                    (0.0, space_between_amount)
                }
                JustifyContent::SpaceAround => {
                    let mut space_between_amount =
                        (container_size.0 as f32 - row_width) / row.len() as f32;
                    if space_between_amount < 0.0 {
                        space_between_amount = 0.0;
                    }
                    (space_between_amount / 2.0, space_between_amount)
                }
                JustifyContent::SpaceEvenly => {
                    let mut space_between_amount =
                        (container_size.0 as f32 - row_width) / (row.len() + 1) as f32;
                    if space_between_amount < 0.0 {
                        space_between_amount = 0.0;
                    }
                    (space_between_amount, space_between_amount)
                }
            };

            x = new_x;
            for widget in row {
                let (widget_y_offset, widget_height) = match self.align_items {
                    AlignItems::FlexStart => (0.0, widget.height),
                    AlignItems::FlexEnd => {
                        let mut widget_y_offset = row_height - widget.height;
                        if widget_y_offset < 0.0 {
                            widget_y_offset = 0.0;
                        }
                        (widget_y_offset, widget.height)
                    }
                    AlignItems::Center => {
                        let mut widget_y_offset = (row_height - widget.height) / 2.0;
                        if widget_y_offset < 0.0 {
                            widget_y_offset = 0.0;
                        }
                        (widget_y_offset, widget.height)
                    }
                    AlignItems::Stretch => {
                        (0.0, row_height.clamp(widget.min_height, widget.max_height))
                    }
                };
                self.widget_subareas.push(Rect::sized(
                    x,
                    y + widget_y_offset,
                    widget.width,
                    widget_height,
                ));
                x += widget.width + x_offset;
            }
            y += row_height + y_offset;
        }

        container
    }

    fn render<'a>(&'a self, mut area: Area<'a>) {
        debug_assert_eq!(self.widgets.len(), self.widget_subareas.len());

        for i in 0..self.widgets.len() {
            // Todo: go unsafe
            let widget = &self.widgets[i];
            let subarea = self.widget_subareas[i];
            let area = area.subarea(subarea);
            widget.render(area);
        }
    }
}

impl std::fmt::Debug for Flexbox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct DString {
            s: String,
        }
        impl std::fmt::Debug for DString {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.s)
            }
        }

        f.debug_struct("Flexbox")
            .field(
                "widgets",
                &DString {
                    s: format!("{} widgets", self.widgets.len()),
                },
            )
            .field("widget_subareas", &self.widget_subareas)
            .field("align_content", &self.align_content)
            .field("align_items", &self.align_items)
            .field("flex_wrap", &self.flex_wrap)
            .field("justify_content", &self.justify_content)
            .field(
                "area_allocator",
                &DString {
                    s: if self.area_allocator.is_some() {
                        "Some(function)".to_string()
                    } else {
                        "None".to_string()
                    },
                },
            )
            .field("must_render", &self.must_render)
            .finish()
    }
}

impl Flexbox {
    pub fn new() -> Flexbox {
        Flexbox {
            widgets: Vec::new(),
            widget_subareas: Vec::new(),
            align_content: AlignContent::FlexStart,
            align_items: AlignItems::Stretch,
            flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            area_allocator: None,
            must_render: true,
        }
    }

    pub fn add(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
        self.must_render = true;
    }

    pub fn set_flex_wrap(&mut self, flex_wrap: FlexWrap) {
        self.flex_wrap = flex_wrap;
        self.must_render = true;
    }

    pub fn set_justify_content(&mut self, justify_content: JustifyContent) {
        self.justify_content = justify_content;
        self.must_render = true;
    }

    pub fn set_align_content(&mut self, align_content: AlignContent) {
        self.align_content = align_content;
        self.must_render = true;
    }

    pub fn set_align_items(&mut self, align_items: AlignItems) {
        self.align_items = align_items;
        self.must_render = true;
    }

    /// By default, the size of the flexbox will be automatically inferred from its content.
    /// The default size will be restricted by its container.
    /// You may want to set a custom area allocator that the flexbox will fit exactly.
    pub fn set_area_allocator(&mut self, area_allocator: Option<AreaAllocator>) {
        self.area_allocator = area_allocator;
        self.must_render = true;
    }
}

impl<'a> Default for Flexbox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Button {}

    impl Widget for Button {
        fn allocate_area(
            &mut self,
            _screen_size: (usize, usize),
            _container_size: (usize, usize),
        ) -> WidgetSize {
            WidgetSize {
                min_width: 40.0,
                width: 50.0,
                max_width: 60.0,
                min_height: 15.0,
                height: 20.0,
                max_height: 25.0,
            }
        }

        fn render(&self, _area: Area) {}
    }

    #[test]
    fn test_flexbox_no_wrap() {
        let mut flexbox = Flexbox::new();
        flexbox.add(Box::new(Button {}));
        flexbox.add(Box::new(Button {}));
        flexbox.add(Box::new(Button {}));

        // Simple behavior
        let size = flexbox.allocate_area((1000, 1000), (200, 100));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 0.0,
                width: 150.0,
                max_width: 200.0,
                min_height: 0.0,
                height: 20.0,
                max_height: 100.0,
            }
        );
        assert_eq!(
            flexbox.widget_subareas,
            vec![
                Rect {
                    top_left: (0.0, 0.0),
                    bottom_right: (50.0, 20.0)
                },
                Rect {
                    top_left: (50.0, 0.0),
                    bottom_right: (100.0, 20.0)
                },
                Rect {
                    top_left: (100.0, 0.0),
                    bottom_right: (150.0, 20.0)
                }
            ]
        );

        // Resize elements
        let size = flexbox.allocate_area((1000, 1000), (100, 15));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 0.0,
                width: 100.0,
                max_width: 100.0,
                min_height: 0.0,
                height: 15.0,
                max_height: 15.0,
            }
        );
        assert_eq!(
            flexbox.widget_subareas,
            vec![
                Rect {
                    top_left: (0.0, 0.0),
                    bottom_right: (40.0, 15.0)
                },
                Rect {
                    top_left: (40.0, 0.0),
                    bottom_right: (80.0, 15.0)
                },
                Rect {
                    top_left: (80.0, 0.0),
                    bottom_right: (120.0, 15.0)
                }
            ]
        );

        // Try to overflow the container of the flexbox
        let size = flexbox.allocate_area((1000, 1000), (10, 10));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 0.0,
                width: 10.0,
                max_width: 10.0,
                min_height: 0.0,
                height: 10.0,
                max_height: 10.0,
            }
        );

        // Test with custom allocator
        flexbox.set_area_allocator(Some(Box::new(|_screen_size, _container_size| WidgetSize {
            min_width: 30.0,
            width: 100.0,
            max_width: 200.0,
            min_height: 10.0,
            height: 10.0,
            max_height: 20.0,
        })));
        let size = flexbox.allocate_area((1000, 1000), (200, 100));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 30.0,
                width: 150.0,
                max_width: 200.0,
                min_height: 10.0,
                height: 20.0,
                max_height: 20.0,
            }
        );
        assert_eq!(
            flexbox.widget_subareas,
            vec![
                Rect {
                    top_left: (0.0, 0.0),
                    bottom_right: (50.0, 20.0)
                },
                Rect {
                    top_left: (50.0, 0.0),
                    bottom_right: (100.0, 20.0)
                },
                Rect {
                    top_left: (100.0, 0.0),
                    bottom_right: (150.0, 20.0)
                }
            ]
        );

        // Try to underflow the allocated box
        flexbox.set_area_allocator(Some(Box::new(|_screen_size, _container_size| WidgetSize {
            min_width: 250.0,
            width: 250.0,
            max_width: 300.0,
            min_height: 50.0,
            height: 50.0,
            max_height: 100.0,
        })));
        let size = flexbox.allocate_area((1000, 1000), (250, 100));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 250.0,
                width: 250.0,
                max_width: 300.0,
                min_height: 50.0,
                height: 50.0,
                max_height: 100.0,
            }
        );
    }

    #[test]
    fn test_flexbox_size_wrap() {
        let mut flexbox = Flexbox::new();
        flexbox.set_flex_wrap(FlexWrap::Wrap);
        flexbox.add(Box::new(Button {}));
        flexbox.add(Box::new(Button {}));
        flexbox.add(Box::new(Button {}));

        // Simple behavior
        let size = flexbox.allocate_area((1000, 1000), (200, 100));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 0.0,
                width: 150.0,
                max_width: 200.0,
                min_height: 0.0,
                height: 20.0,
                max_height: 100.0,
            }
        );
        assert_eq!(
            flexbox.widget_subareas,
            vec![
                Rect {
                    top_left: (0.0, 0.0),
                    bottom_right: (50.0, 20.0)
                },
                Rect {
                    top_left: (50.0, 0.0),
                    bottom_right: (100.0, 20.0)
                },
                Rect {
                    top_left: (100.0, 0.0),
                    bottom_right: (150.0, 20.0)
                }
            ]
        );

        // Test wrapping
        let size = flexbox.allocate_area((1000, 1000), (100, 100));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 0.0,
                width: 100.0,
                max_width: 100.0,
                min_height: 0.0,
                height: 40.0,
                max_height: 100.0,
            }
        );
        assert_eq!(
            flexbox.widget_subareas,
            vec![
                Rect {
                    top_left: (0.0, 0.0),
                    bottom_right: (50.0, 20.0)
                },
                Rect {
                    top_left: (50.0, 0.0),
                    bottom_right: (100.0, 20.0)
                },
                Rect {
                    top_left: (0.0, 20.0),
                    bottom_right: (50.0, 40.0)
                }
            ]
        );

        // Test wrapping + resizing
        let size = flexbox.allocate_area((1000, 1000), (45, 100));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 0.0,
                width: 45.0,
                max_width: 45.0,
                min_height: 0.0,
                height: 60.0,
                max_height: 100.0,
            }
        );
        assert_eq!(
            flexbox.widget_subareas,
            vec![
                Rect {
                    top_left: (0.0, 0.0),
                    bottom_right: (45.0, 20.0)
                },
                Rect {
                    top_left: (0.0, 20.0),
                    bottom_right: (45.0, 40.0)
                },
                Rect {
                    top_left: (0.0, 40.0),
                    bottom_right: (45.0, 60.0)
                }
            ]
        );

        // Try to overflow
        let size = flexbox.allocate_area((1000, 1000), (10, 10));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 0.0,
                width: 10.0,
                max_width: 10.0,
                min_height: 0.0,
                height: 10.0,
                max_height: 10.0,
            }
        );
        assert_eq!(
            flexbox.widget_subareas,
            vec![
                Rect {
                    top_left: (0.0, 0.0),
                    bottom_right: (40.0, 15.0)
                },
                Rect {
                    top_left: (0.0, 15.0),
                    bottom_right: (40.0, 30.0)
                },
                Rect {
                    top_left: (0.0, 30.0),
                    bottom_right: (40.0, 45.0)
                }
            ]
        );

        // Test with custom allocator
        flexbox.set_area_allocator(Some(Box::new(|_screen_size, _container_size| WidgetSize {
            min_width: 30.0,
            width: 45.0,
            max_width: 55.0,
            min_height: 10.0,
            height: 10.0,
            max_height: 100.0,
        })));
        let size = flexbox.allocate_area((1000, 1000), (200, 100));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 30.0,
                width: 50.0,
                max_width: 55.0,
                min_height: 10.0,
                height: 60.0,
                max_height: 100.0,
            }
        );
        assert_eq!(
            flexbox.widget_subareas,
            vec![
                Rect {
                    top_left: (0.0, 0.0),
                    bottom_right: (50.0, 20.0)
                },
                Rect {
                    top_left: (0.0, 20.0),
                    bottom_right: (50.0, 40.0)
                },
                Rect {
                    top_left: (0.0, 40.0),
                    bottom_right: (50.0, 60.0)
                }
            ]
        );

        // Try to underflow the allocated box
        flexbox.set_area_allocator(Some(Box::new(|_screen_size, _container_size| WidgetSize {
            min_width: 150.0,
            width: 150.0,
            max_width: 200.0,
            min_height: 80.0,
            height: 80.0,
            max_height: 100.0,
        })));
        let size = flexbox.allocate_area((1000, 1000), (200, 100));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 150.0,
                width: 150.0,
                max_width: 200.0,
                min_height: 80.0,
                height: 80.0,
                max_height: 100.0,
            }
        );
    }
}
