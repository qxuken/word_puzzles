use crate::assets::AssetsMetadataStore;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub is_dev: bool,
    pub assets_metadata: AssetsMetadataStore,
}

pub type SharedAppState = Arc<AppState>;

impl AppState {
    pub fn new(is_dev: bool) -> Self {
        let assets_metadata = AssetsMetadataStore::new(is_dev);

        Self {
            is_dev,
            assets_metadata,
        }
    }

    pub fn shared(self) -> SharedAppState {
        Arc::new(self)
    }
}
