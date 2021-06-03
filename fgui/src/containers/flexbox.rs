use crate::*;
use containers::*;

/// A function that takes the screen size and the container size, returning the required size of an item.
type AreaAllocator = Box<dyn FnMut((usize, usize), (usize, usize)) -> WidgetSize>;

pub struct Flexbox<Backend: BackendTrait> {
    widgets: Vec<Box<dyn Widget<Backend>>>,
    widget_subareas: Vec<Rect>,
    align_content: AlignContent,
    align_items: AlignItems,
    flex_wrap: FlexWrap,
    justify_content: JustifyContent,
    area_allocator: Option<AreaAllocator>,
    must_render: bool,
}

impl<Backend: BackendTrait> Widget<Backend> for Flexbox<Backend> {
    fn accept_render(&self) -> bool {
        if self.must_render {
            todo!();
            // self.must_render = false;
            // return true;
        }
        for widget in &self.widgets {
            if widget.accept_render() {
                return true;
            }
        }
        false
    }

    fn allocate_area(
        &mut self,
        screen: (usize, usize),
        container_size: (usize, usize),
    ) -> WidgetSize {
        use std::cmp::{max, min};

        // Get the size restrictions of the flexbox
        let mut container = match &mut self.area_allocator {
            Some(area_allocator) => area_allocator(screen, container_size),
            None => WidgetSize {
                min_width: 0,
                width: 0,
                max_width: container_size.0,
                min_height: 0,
                height: 0,
                max_height: container_size.1,
            },
        };

        // Get the size of the widgets and wrap them if needed
        let mut rows = Vec::new();
        match self.flex_wrap {
            FlexWrap::NoWrap => {
                let (mut row, mut row_width, mut row_height) = (Vec::new(), 0, 0);
                for widget in &mut self.widgets {
                    let widget_size =
                        widget.allocate_area(screen, (container.max_width, container.max_height));
                    row_width += widget_size.width;
                    row_height = max(row_height, widget_size.height);
                    row.push(widget_size);
                }
                rows.push((row, row_width, row_height));
            }
            FlexWrap::Wrap => {
                let (mut row, mut row_width, mut row_height) = (Vec::new(), 0, 0);
                for widget in &mut self.widgets {
                    let widget_size =
                        widget.allocate_area(screen, (container.max_width, container.max_height));

                    if row_width + widget_size.width > container.max_width {
                        rows.push((row, row_width, row_height));
                        row_width = 0;
                        row_height = 0;
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
                let mut total_shrink_capacity = 0;
                for widget in widgets.iter() {
                    total_shrink_capacity += widget.width - widget.min_width;
                }

                // Shrink the sizes as much as needed
                *row_width = 0;
                for widget in widgets.iter_mut() {
                    let shrink_capacity = widget.width - widget.min_width;
                    let factor = (overflow as f32 / total_shrink_capacity as f32).clamp(0.0, 1.0);
                    let to_skrink = (shrink_capacity as f32 * factor) as usize;

                    overflow -= to_skrink;
                    widget.width -= to_skrink;
                    total_shrink_capacity -= shrink_capacity;
                    *row_width += widget.width;
                }
            }
        }

        // Resize to fix vertical overflows
        // The rows will try to fit in the space
        let mut flexbox_height = 0;
        for (_, _, row_height) in &rows {
            flexbox_height += row_height;
        }
        if flexbox_height > container.max_height {
            // Calculate the overflow and calculate the shrinkable size
            let mut overflow = flexbox_height - container.max_height;
            let mut shrinkables = Vec::new();
            for (row, _, row_height) in &mut rows {
                let mut min_row_height = 0;
                for widget in row.iter() {
                    min_row_height = max(min_row_height, widget.min_height);
                }
                shrinkables.push((*row_height - min_row_height, row, row_height));
            }
            shrinkables.sort_by_key(|(amount, _, _)| *amount);

            // Shrink the sizes as much as possible until the size fits
            let mut remaining_blocks = shrinkables.len();
            for (mut amount, row, row_height) in shrinkables {
                let to_skrink = overflow / remaining_blocks;
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
        let mut flexbox_width = 0;
        flexbox_height = 0;
        for (_, row_width, row_height) in &rows {
            flexbox_width = max(flexbox_width, *row_width);
            flexbox_height += row_height;
        }

        // Prepare subareas
        let mut x = 0;
        let mut y = 0;
        self.widget_subareas.clear();
        for (row, _, row_height) in &rows {
            for widget in row {
                self.widget_subareas.push(Rect::sized(
                    x as isize,
                    y as isize,
                    widget.width,
                    widget.height,
                ));
                x += widget.width;
            }
            y += row_height;
            x = 0;
        }

        container.set_size(flexbox_width, flexbox_height);
        container
    }

    fn render<'a>(&'a self, mut area: Area<'a, Backend>) {
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

impl<Backend: BackendTrait> Flexbox<Backend> {
    pub fn new() -> Flexbox<Backend> {
        Flexbox {
            widgets: Vec::new(),
            widget_subareas: Vec::new(),
            align_content: AlignContent::Normal,
            align_items: AlignItems::Stretch,
            flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            area_allocator: None,
            must_render: true,
        }
    }

    pub fn add(&mut self, widget: Box<dyn Widget<Backend>>) {
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

impl<'a, Backend: BackendTrait> Default for Flexbox<Backend> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestBackend {}

    impl Backend for TestBackend {
        fn run(_app: impl App<Self>) -> ! {
            todo!()
        }
    }

    struct Button {}

    impl Widget<TestBackend> for Button {
        fn accept_render(&self) -> bool {
            true
        }

        fn allocate_area(
            &mut self,
            _screen_size: (usize, usize),
            _container_size: (usize, usize),
        ) -> WidgetSize {
            WidgetSize {
                min_width: 40,
                width: 50,
                max_width: 60,
                min_height: 15,
                height: 20,
                max_height: 25,
            }
        }

        fn render(&self, _area: Area<TestBackend>) {}
    }

    #[test]
    fn test_flexbox_no_wrap() {
        let mut flexbox = Flexbox::<TestBackend>::new();
        flexbox.add(Box::new(Button {}));
        flexbox.add(Box::new(Button {}));
        flexbox.add(Box::new(Button {}));

        // Simple behavior
        let size = flexbox.allocate_area((1000, 1000), (200, 100));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 0,
                width: 150,
                max_width: 200,
                min_height: 0,
                height: 20,
                max_height: 100,
            }
        );
        assert_eq!(
            flexbox.widget_subareas,
            vec![
                Rect {
                    top_left: (0, 0),
                    bottom_right: (50, 20)
                },
                Rect {
                    top_left: (50, 0),
                    bottom_right: (100, 20)
                },
                Rect {
                    top_left: (100, 0),
                    bottom_right: (150, 20)
                }
            ]
        );

        // Resize elements
        let size = flexbox.allocate_area((1000, 1000), (100, 15));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 0,
                width: 100,
                max_width: 100,
                min_height: 0,
                height: 15,
                max_height: 15,
            }
        );
        assert_eq!(
            flexbox.widget_subareas,
            vec![
                Rect {
                    top_left: (0, 0),
                    bottom_right: (40, 15)
                },
                Rect {
                    top_left: (40, 0),
                    bottom_right: (80, 15)
                },
                Rect {
                    top_left: (80, 0),
                    bottom_right: (120, 15)
                }
            ]
        );

        // Try to overflow the container of the flexbox
        let size = flexbox.allocate_area((1000, 1000), (10, 10));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 0,
                width: 10,
                max_width: 10,
                min_height: 0,
                height: 10,
                max_height: 10,
            }
        );

        // Test with custom allocator
        flexbox.set_area_allocator(Some(Box::new(|_screen_size, _container_size| WidgetSize {
            min_width: 30,
            width: 100,
            max_width: 200,
            min_height: 10,
            height: 10,
            max_height: 20,
        })));
        let size = flexbox.allocate_area((1000, 1000), (200, 100));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 30,
                width: 150,
                max_width: 200,
                min_height: 10,
                height: 20,
                max_height: 20,
            }
        );
        assert_eq!(
            flexbox.widget_subareas,
            vec![
                Rect {
                    top_left: (0, 0),
                    bottom_right: (50, 20)
                },
                Rect {
                    top_left: (50, 0),
                    bottom_right: (100, 20)
                },
                Rect {
                    top_left: (100, 0),
                    bottom_right: (150, 20)
                }
            ]
        );

        // Try to underflow the allocated box
        flexbox.set_area_allocator(Some(Box::new(|_screen_size, _container_size| WidgetSize {
            min_width: 250,
            width: 250,
            max_width: 300,
            min_height: 50,
            height: 50,
            max_height: 100,
        })));
        let size = flexbox.allocate_area((1000, 1000), (250, 100));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 250,
                width: 250,
                max_width: 300,
                min_height: 50,
                height: 50,
                max_height: 100,
            }
        );
    }

    #[test]
    fn test_flexbox_size_wrap() {
        let mut flexbox = Flexbox::<TestBackend>::new();
        flexbox.set_flex_wrap(FlexWrap::Wrap);
        flexbox.add(Box::new(Button {}));
        flexbox.add(Box::new(Button {}));
        flexbox.add(Box::new(Button {}));

        // Simple behavior
        let size = flexbox.allocate_area((1000, 1000), (200, 100));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 0,
                width: 150,
                max_width: 200,
                min_height: 0,
                height: 20,
                max_height: 100,
            }
        );
        assert_eq!(
            flexbox.widget_subareas,
            vec![
                Rect {
                    top_left: (0, 0),
                    bottom_right: (50, 20)
                },
                Rect {
                    top_left: (50, 0),
                    bottom_right: (100, 20)
                },
                Rect {
                    top_left: (100, 0),
                    bottom_right: (150, 20)
                }
            ]
        );

        // Test wrapping
        let size = flexbox.allocate_area((1000, 1000), (100, 100));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 0,
                width: 100,
                max_width: 100,
                min_height: 0,
                height: 40,
                max_height: 100,
            }
        );
        assert_eq!(
            flexbox.widget_subareas,
            vec![
                Rect {
                    top_left: (0, 0),
                    bottom_right: (50, 20)
                },
                Rect {
                    top_left: (50, 0),
                    bottom_right: (100, 20)
                },
                Rect {
                    top_left: (0, 20),
                    bottom_right: (50, 40)
                }
            ]
        );

        // Test wrapping + resizing
        let size = flexbox.allocate_area((1000, 1000), (45, 100));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 0,
                width: 45,
                max_width: 45,
                min_height: 0,
                height: 60,
                max_height: 100,
            }
        );
        assert_eq!(
            flexbox.widget_subareas,
            vec![
                Rect {
                    top_left: (0, 0),
                    bottom_right: (45, 20)
                },
                Rect {
                    top_left: (0, 20),
                    bottom_right: (45, 40)
                },
                Rect {
                    top_left: (0, 40),
                    bottom_right: (45, 60)
                }
            ]
        );

        // Try to overflow
        let size = flexbox.allocate_area((1000, 1000), (10, 10));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 0,
                width: 10,
                max_width: 10,
                min_height: 0,
                height: 10,
                max_height: 10,
            }
        );
        assert_eq!(
            flexbox.widget_subareas,
            vec![
                Rect {
                    top_left: (0, 0),
                    bottom_right: (40, 15)
                },
                Rect {
                    top_left: (0, 15),
                    bottom_right: (40, 30)
                },
                Rect {
                    top_left: (0, 30),
                    bottom_right: (40, 45)
                }
            ]
        );

        // Test with custom allocator
        flexbox.set_area_allocator(Some(Box::new(|_screen_size, _container_size| WidgetSize {
            min_width: 30,
            width: 45,
            max_width: 55,
            min_height: 10,
            height: 10,
            max_height: 100,
        })));
        let size = flexbox.allocate_area((1000, 1000), (200, 100));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 30,
                width: 50,
                max_width: 55,
                min_height: 10,
                height: 60,
                max_height: 100,
            }
        );
        assert_eq!(
            flexbox.widget_subareas,
            vec![
                Rect {
                    top_left: (0, 0),
                    bottom_right: (50, 20)
                },
                Rect {
                    top_left: (0, 20),
                    bottom_right: (50, 40)
                },
                Rect {
                    top_left: (0, 40),
                    bottom_right: (50, 60)
                }
            ]
        );

        // Try to underflow the allocated box
        flexbox.set_area_allocator(Some(Box::new(|_screen_size, _container_size| WidgetSize {
            min_width: 150,
            width: 150,
            max_width: 200,
            min_height: 80,
            height: 80,
            max_height: 100,
        })));
        let size = flexbox.allocate_area((1000, 1000), (200, 100));
        assert_eq!(
            size,
            WidgetSize {
                min_width: 150,
                width: 150,
                max_width: 200,
                min_height: 80,
                height: 80,
                max_height: 100,
            }
        );
    }
}
