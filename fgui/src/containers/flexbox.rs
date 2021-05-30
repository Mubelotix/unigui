use crate::*;
use containers::*;

/// A function that takes the screen size and the container size, returning the required size of an item.
type AreaAllocator = Box<dyn FnMut((usize, usize), (usize, usize)) -> WidgetSize>;

pub struct Flexbox<Backend: BackendTrait> {
    widgets: Vec<Box<dyn Widget<Backend>>>,
    widget_sizes: Vec<WidgetSize>,
    align_content: AlignContent,
    align_items: AlignItems,
    flex_wrap: FlexWrap,
    justify_content: JustifyContent,
    area_allocator: Option<AreaAllocator>,
    must_render: bool,
}

impl<Backend: BackendTrait> Widget<Backend> for Flexbox<Backend> {
    fn accept_render(&mut self) -> bool {
        if self.must_render {
            self.must_render = false;
            return true;
        }
        for widget in &mut self.widgets {
            if widget.accept_render() {
                return true;
            }
        }
        false
    }

    fn allocate_area(
        &mut self,
        screen_size: (usize, usize),
        container_size: (usize, usize),
    ) -> WidgetSize {
        let size_restriction = match &mut self.area_allocator {
            Some(area_allocator) => area_allocator(screen_size, container_size),
            None => WidgetSize {
                min_width: 0,
                width: 0,
                max_width: container_size.0,
                min_height: 0,
                height: 0,
                max_height: container_size.1,
            },
        };

        self.widget_sizes.clear();
        for widget in &mut self.widgets {
            self.widget_sizes
                .push(widget.allocate_area(screen_size, (size_restriction.max_width, size_restriction.max_height)));
        }

        match self.flex_wrap {
            FlexWrap::NoWrap => self.allocate_area_no_wrap(size_restriction),
            FlexWrap::Wrap => self.allocate_area_wrap(size_restriction),
            FlexWrap::WrapReverse => {
                todo!()
            }
        }
    }

    fn render(&self, _area: Area<Backend>) {
        todo!()
    }
}

impl<Backend: BackendTrait> Flexbox<Backend> {
    fn allocate_area_no_wrap(&mut self, mut size_restriction: WidgetSize) -> WidgetSize {
        let mut flexbox_width = 0;
        let mut flexbox_height = 0;
        for size in &self.widget_sizes {
            flexbox_width += size.width;
            if size.height > flexbox_height {
                flexbox_height = size.height;
            }
        }

        // todo flex grow

        if size_restriction.max_width < flexbox_width {
            // Calculate the overflow and calculate the shrinkable size
            let mut overflow = flexbox_width - size_restriction.max_width;
            let mut shrinkables = Vec::new();
            for size in &mut self.widget_sizes {
                let shrinkable = size.width - size.min_width;
                shrinkables.push((shrinkable, size));
            }
            shrinkables.sort_by_key(|(amount, _)| *amount);

            // Shrink the sizes as much as possible until the size fits
            let mut remaining_blocks = shrinkables.len();
            for (mut shrinkable, size) in shrinkables {
                let to_skrink = overflow / remaining_blocks;
                shrinkable = std::cmp::min(shrinkable, to_skrink);
                size.width -= shrinkable;
                overflow -= shrinkable;

                remaining_blocks -= 1;
            }

            // Recaculate the size
            flexbox_width = 0;
            flexbox_height = 0;
            for size in &self.widget_sizes {
                flexbox_width += size.width;
                if size.height > flexbox_height {
                    flexbox_height = size.height;
                }
            }
        }

        size_restriction.set_size(flexbox_width, flexbox_height);
        size_restriction
    }

    fn allocate_area_wrap(&mut self, mut size_restriction: WidgetSize) -> WidgetSize {
        let mut flexbox_line_size = vec![(0, 0)];
        for size in &mut self.widget_sizes {
            let last_line_size = flexbox_line_size.last_mut().unwrap(); // TODO: unwrap_unchecked
            let height = size.height.clamp(size_restriction.min_height, size_restriction.max_height);
            if last_line_size.0 + size.width > size_restriction.max_width {
                let width = size.width.clamp(size_restriction.min_width, size_restriction.max_width);
                flexbox_line_size.push((width, height));
            } else {
                last_line_size.0 += size.width;
                last_line_size.1 = std::cmp::max(last_line_size.1, height);
            }
        }

        let mut flexbox_width = 0;
        let mut flexbox_height = 0;
        for line_size in flexbox_line_size {
            flexbox_width = std::cmp::max(flexbox_width, line_size.0);
            flexbox_height += line_size.1;
        }

        size_restriction.set_size(flexbox_width, flexbox_height);
        size_restriction
    }
}

impl<Backend: BackendTrait> Flexbox<Backend> {
    pub fn new() -> Flexbox<Backend> {
        Flexbox {
            widgets: Vec::new(),
            widget_sizes: Vec::new(),
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
        fn init() -> Self {
            TestBackend {}
        }

        fn clear(&mut self) -> Area<Self> {
            Area::new(Rect::sized(0, 0, 1000, 1000), self)
        }
    }

    struct Button {}

    impl Widget<TestBackend> for Button {
        fn accept_render(&mut self) -> bool {
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
    fn test_flexbox_size_no_wrap() {
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

        // Test with custom allocator
        flexbox.set_area_allocator(Some(Box::new(|_screen_size, _container_size| WidgetSize {
            min_width: 30,
            width: 100,
            max_width: 50,
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
                max_width: 50,
                min_height: 10,
                height: 60,
                max_height: 100,
            }
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
