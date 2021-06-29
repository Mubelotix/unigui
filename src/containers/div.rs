use crate::prelude::*;
use containers::*;

/// A very simple container that will basically act as an HTML div.
pub struct Div {
    widgets: Vec<(Box<dyn Widget>, bool)>, // Vec<(widget, is_inline)>
    widget_subareas: Vec<Rect>,
    area_allocator: Option<AreaAllocator>,
}

impl Div {
    pub fn new() -> Div {
        Div {
            widgets: Vec::new(),
            widget_subareas: Vec::new(),
            area_allocator: None,
        }
    }

    /// Add a widget after already-added widgets.  
    /// A line return will be inserted before this widget.
    pub fn push_block(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push((widget, false));
    }

    /// Add a widget after already-added widgets.  
    /// This widget will be placed directly at the right of the last widget, without line return.
    /// If the widget does not fit at this place, the line will be wrapped.
    pub fn push_inline(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push((widget, true));
    }

    /// Alias for [Div::push_block].
    pub fn add_block(&mut self, widget: Box<dyn Widget>) {
        self.push_block(widget);
    }

    /// Alias for [Div::push_inline].
    pub fn add_inline(&mut self, widget: Box<dyn Widget>) {
        self.push_inline(widget);
    }

    /// By default, the size of the div will be automatically inferred from its content.
    /// The default size will be restricted by its container and be as small as possible.
    /// You may want to set a custom area allocator that the div will fit exactly.
    pub fn set_area_allocator(&mut self, area_allocator: Option<AreaAllocator>) {
        self.area_allocator = area_allocator;
    }
}

impl Widget for Div {
    fn allocate_area(
        &mut self,
        screen_size: (usize, usize),
        container_size: (usize, usize),
    ) -> WidgetSize {
        // Get the size restrictions of the flexbox
        let mut container = match &mut self.area_allocator {
            Some(area_allocator) => area_allocator(screen_size, container_size),
            None => WidgetSize {
                min_width: 0.0,
                width: 0.0,
                max_width: container_size.0 as f32,
                min_height: 0.0,
                height: 0.0,
                max_height: container_size.1 as f32,
            },
        };

        // Place widgets in rows
        let mut rows: Vec<(f32, f32, Vec<WidgetSize>)> = Vec::new();
        let mut previous_is_inline = true;
        for (widget, is_inline) in &mut self.widgets {
            let mut widget_size = widget.allocate_area(screen_size, container_size);
            if *is_inline
                && previous_is_inline
                && rows.last().map(|(w, _, _)| *w).unwrap_or(0.0) + widget_size.width
                    <= container.max_width
            {
                match rows.last_mut() {
                    Some((row_width, row_height, widget_sizes)) => {
                        *row_height = max(*row_height, widget_size.height);
                        *row_width += widget_size.width;
                        widget_sizes.push(widget_size);
                    }
                    None => {
                        rows.push((widget_size.width, widget_size.height, vec![widget_size]));
                    }
                };
            } else {
                widget_size.fit_width_in(&container);
                widget_size.fit_height_in(&container);
                rows.push((widget_size.width, widget_size.height, vec![widget_size]));
            }
            previous_is_inline = *is_inline;
        }

        // Prepare subareas
        self.widget_subareas.clear();
        let mut div_width = 0.0;
        let mut x;
        let mut y = 0.0;
        for (row_width, row_height, widgets) in rows {
            div_width = max(div_width, row_width);
            x = 0.0;
            for widget in widgets {
                self.widget_subareas
                    .push(Rect::sized(x, y, widget.width, widget.height));
                x += widget.width;
            }
            y += row_height;
        }

        container.set_size(div_width, y);
        container
    }

    fn render(&self, mut area: Area) {
        debug_assert_eq!(self.widgets.len(), self.widget_subareas.len());

        for i in 0..self.widgets.len() {
            // Todo: go unsafe
            let widget = &self.widgets[i].0;
            let subarea = self.widget_subareas[i];
            let area = area.subarea(subarea);
            widget.render(area);
        }
    }
}

impl Default for Div {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Div {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct DString {
            s: String,
        }
        impl std::fmt::Debug for DString {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.s)
            }
        }

        f.debug_struct("Div")
            .field(
                "widgets",
                &DString {
                    s: format!("{} widgets", self.widgets.len()),
                },
            )
            .field("widget_subareas", &self.widget_subareas)
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
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rect::Rect;

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
    fn test_div_basic() {
        let mut div = Div::new();
        div.add_block(Box::new(Button {}));
        div.add_block(Box::new(Button {}));
        div.add_inline(Box::new(Button {}));
        div.add_inline(Box::new(Button {}));
        div.add_inline(Box::new(Button {}));

        div.allocate_area((1000, 1000), (1000, 1000));
        assert_eq!(
            div.widget_subareas,
            vec![
                Rect {
                    min: (0.0, 0.0),
                    max: (50.0, 20.0),
                },
                Rect {
                    min: (0.0, 20.0),
                    max: (50.0, 40.0),
                },
                Rect {
                    min: (0.0, 40.0),
                    max: (50.0, 60.0),
                },
                Rect {
                    min: (50.0, 40.0),
                    max: (100.0, 60.0),
                },
                Rect {
                    min: (100.0, 40.0),
                    max: (150.0, 60.0),
                },
            ]
        );
    }

    #[test]
    fn test_div_wrapping() {
        let mut div = Div::new();
        div.add_block(Box::new(Button {}));
        div.add_block(Box::new(Button {}));
        div.add_inline(Box::new(Button {}));
        div.add_inline(Box::new(Button {}));
        div.add_inline(Box::new(Button {}));

        div.allocate_area((100, 100), (100, 100));
        assert_eq!(
            div.widget_subareas,
            vec![
                Rect {
                    min: (0.0, 0.0),
                    max: (50.0, 20.0),
                },
                Rect {
                    min: (0.0, 20.0),
                    max: (50.0, 40.0),
                },
                Rect {
                    min: (0.0, 40.0),
                    max: (50.0, 60.0),
                },
                Rect {
                    min: (50.0, 40.0),
                    max: (100.0, 60.0),
                },
                Rect {
                    min: (0.0, 60.0),
                    max: (50.0, 80.0),
                },
            ]
        );
    }

    #[test]
    fn test_div_resizing() {
        let mut div = Div::new();
        div.add_block(Box::new(Button {}));
        div.add_block(Box::new(Button {}));
        div.add_inline(Box::new(Button {}));
        div.add_inline(Box::new(Button {}));
        div.add_inline(Box::new(Button {}));

        div.allocate_area((45, 100), (45, 100));
        assert_eq!(
            div.widget_subareas,
            vec![
                Rect {
                    min: (0.0, 0.0),
                    max: (45.0, 20.0),
                },
                Rect {
                    min: (0.0, 20.0),
                    max: (45.0, 40.0),
                },
                Rect {
                    min: (0.0, 40.0),
                    max: (45.0, 60.0),
                },
                Rect {
                    min: (0.0, 60.0),
                    max: (45.0, 80.0),
                },
                Rect {
                    min: (0.0, 80.0),
                    max: (45.0, 100.0),
                },
            ]
        );

        div.allocate_area((35, 50), (35, 50));
        assert_eq!(
            div.widget_subareas,
            vec![
                Rect {
                    min: (0.0, 0.0),
                    max: (40.0, 20.0),
                },
                Rect {
                    min: (0.0, 20.0),
                    max: (40.0, 40.0),
                },
                Rect {
                    min: (0.0, 40.0),
                    max: (40.0, 60.0),
                },
                Rect {
                    min: (0.0, 60.0),
                    max: (40.0, 80.0),
                },
                Rect {
                    min: (0.0, 80.0),
                    max: (40.0, 100.0),
                },
            ]
        );
    }

    #[test]
    fn test_div_custom_allocator() {
        let mut div = Div::new();
        div.set_area_allocator(Some(Box::new(|_, _| WidgetSize {
            min_width: 200.0,
            width: 200.0,
            max_width: 200.0,
            min_height: 200.0,
            height: 200.0,
            max_height: 200.0,
        })));
        div.add_block(Box::new(Button {}));
        div.add_block(Box::new(Button {}));
        div.add_inline(Box::new(Button {}));
        div.add_inline(Box::new(Button {}));
        div.add_inline(Box::new(Button {}));

        div.allocate_area((20, 20), (20, 20));
        assert_eq!(
            div.widget_subareas,
            vec![
                Rect {
                    min: (0.0, 0.0),
                    max: (50.0, 20.0),
                },
                Rect {
                    min: (0.0, 20.0),
                    max: (50.0, 40.0),
                },
                Rect {
                    min: (0.0, 40.0),
                    max: (50.0, 60.0),
                },
                Rect {
                    min: (50.0, 40.0),
                    max: (100.0, 60.0),
                },
                Rect {
                    min: (100.0, 40.0),
                    max: (150.0, 60.0),
                },
            ]
        );
    }
}
