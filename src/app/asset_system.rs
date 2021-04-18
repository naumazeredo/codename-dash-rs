use std::path::Path;
use std::collections::BTreeMap;

use super::{
    App,
    renderer::{
        texture::{Texture, load_texture},
    },
    utils::string_ref::StringRef,
};

pub(super) struct AssetSystem {
    textures: BTreeMap<StringRef, Texture>,
}

impl AssetSystem {
    pub(super) fn new() -> Self {
        Self {
            textures: BTreeMap::new(),
        }
    }

    fn get_texture<P: AsRef<Path>>(&mut self, path: P) -> Texture {
        *self.textures
            .entry(StringRef::new(path.as_ref().display().to_string()))
            .or_insert_with(|| load_texture(path))
    }
}

impl<S> App<'_, S> {
    // @Refactor this is bad since we always have to take the whole path string.
    //           We should get a StringRef instead and not use Path if not necessary
    pub fn get_texture<P: AsRef<Path>>(&mut self, path: P) -> Texture {
        self.asset_system.get_texture(path)
    }
}
