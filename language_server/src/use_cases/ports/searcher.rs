use std::path::Path;

use crate::{
    entities::{FileState, State},
    use_cases::ports::RegexManager,
};

pub trait Searcher {
    fn search_in_path<P: AsRef<Path>>(&self, file: P) -> Option<FileState>;

    fn search_in_text(&self, text: &str) -> Option<FileState>;

    fn recurssive_search<P: AsRef<Path>>(&self, root: P) -> State;

    fn should_skip<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().file_name().map_or_else(
            || true,
            |name| {
                name == "."
                    || name == ".."
                    || name == "node_modules"
                    || name == "target"
                    || name == ".git"
            },
        )
    }
}

pub trait RegexSearcher: Searcher + RegexManager {}
