use std::path::Path;

use crate::{
    entities::{FileState, State, TodoResult},
    use_cases::ports::RegexManager,
};

use super::ports::Searcher;

pub struct Search<T> {
    inner: T,
}

impl<T> Search<T> {
    pub fn new(v: T) -> Self {
        Self { inner: v }
    }
}

impl<T: Searcher> Search<T> {
    pub fn search_in_path<P: AsRef<Path>>(&self, file: P) -> Option<FileState> {
        self.inner.search_in_path(file)
    }

    pub fn search_in_text(&self, text: &str) -> Option<FileState> {
        self.inner.search_in_text(text)
    }

    pub fn recurssive_search<P: AsRef<Path>>(&self, root: P) -> State {
        self.inner.recurssive_search(root)
    }
}

impl<T: RegexManager> Search<T> {
    pub fn update_regex(&mut self, key_words: &[&str]) -> TodoResult<()> {
        self.inner.update_regex(key_words)
    }
}
