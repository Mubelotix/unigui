use crate::*;

pub struct Area<'a> {
    pub rect: Rect, // TODO visibility
    pub backend: &'a mut WgpuBackend,
}

impl<'a> Area<'a> {
    pub fn new(rect: Rect, backend: &'a mut WgpuBackend) -> Area<'a> {
        Area { rect, backend }
    }

    pub fn width(&self) -> f32 {
        self.rect.max.0 - self.rect.min.0
    }

    pub fn height(&self) -> f32 {
        self.rect.max.1 - self.rect.min.1
    }

    pub fn subarea(&mut self, rect: Rect) -> Area {
        Area {
            rect: Rect {
                min: (
                    self.rect.min.0 + rect.min.0,
                    self.rect.min.1 + rect.min.1,
                ),
                max: (
                    self.rect.min.0 + rect.max.0,
                    self.rect.min.1 + rect.max.1,
                ),
            },
            backend: self.backend,
        }
    }
}

impl<'a> std::fmt::Debug for Area<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Area").field("rect", &self.rect).finish() // TODO replace by finish_non_exhaustive once CI accepts it
    }
}

#[allow(clippy::uninit_assumed_init)]
#[allow(invalid_value)]
#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_subarea() {
        let mut area = Area {
            rect: Rect {
                min: (0.0, 0.0),
                max: (1920.0, 1080.0),
            },
            backend: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
        };
        let subarea = area.subarea(Rect {
            min: (10.0, 10.0),
            max: (20.0, 20.0),
        });
        assert_eq!(
            subarea.rect,
            Rect {
                min: (10.0, 10.0),
                max: (20.0, 20.0),
            }
        );

        let mut area = Area {
            rect: Rect {
                min: (10.0, 10.0),
                max: (1920.0, 1080.0),
            },
            backend: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
        };
        let subarea = area.subarea(Rect {
            min: (10.0, 10.0),
            max: (20.0, 20.0),
        });
        assert_eq!(
            subarea.rect,
            Rect {
                min: (20.0, 20.0),
                max: (30.0, 30.0),
            }
        );
    }
}
