use words::{WordsDict, WordsShortcuts};

use crate::assets::AssetsMetadataStore;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub is_dev: bool,
    pub assets_metadata: AssetsMetadataStore,
    pub words_dict: WordsDict,
    pub words_shortcuts: WordsShortcuts,
}

pub type SharedAppState = Arc<AppState>;

impl AppState {
    pub fn new(is_dev: bool) -> Self {
        let assets_metadata = AssetsMetadataStore::new(is_dev);
        let words_dict = WordsDict::load();
        let words_shortcuts = WordsShortcuts::new(&words_dict);

        Self {
            is_dev,
            assets_metadata,
            words_dict,
            words_shortcuts,
        }
    }

    pub fn shared(self) -> SharedAppState {
        Arc::new(self)
    }
}
