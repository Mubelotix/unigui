pub use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TextureId {
    pub(super) id: Arc<usize>,
}

impl TextureId {
    pub(super) fn new(id: usize) -> TextureId {
        TextureId { id: Arc::new(id) }
    }
}
