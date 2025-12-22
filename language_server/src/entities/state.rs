use std::collections::HashMap;

use crate::entities::FileState;

#[derive(Debug, Default)]
pub struct State {
    inner: HashMap<String, FileState>,
}

impl State {
    pub fn extend(&mut self, other: Self) {
        self.inner.extend(other.inner);
    }

    pub fn insert(&mut self, file_name: String, matches: FileState) {
        self.inner.insert(file_name, matches);
    }

    pub fn replace(&mut self, file_name: String, new_matches: FileState) {
        self.insert(file_name, new_matches);
    }

    pub fn from_file_matches(file_name: String, matches: FileState) -> Self {
        let mut inner = HashMap::with_capacity(1);
        inner.insert(file_name, matches);

        Self { inner }
    }

    pub fn get(&self, file_name: &str) -> Option<&FileState> {
        self.inner.get(file_name)
    }

    pub fn remove(&mut self, file_name: &str) {
        let _ = self.inner.remove(file_name);
    }
}
