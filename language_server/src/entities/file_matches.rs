use std::collections::HashMap;

use getset::Getters;

use crate::entities::{Column, Row, RowMetadata};

/// Represents a match of one of the user-defined keywords inside a row
#[derive(Debug, Getters)]
pub struct Match {
    /// The column inside the row where the match starts
    #[getset(get = "pub")]
    column: Column,
    /// User configures keyword that triggered the match
    #[getset(get = "pub")]
    keyword: String,
}

impl Match {
    pub fn new(column: Column, matched_patch: String) -> Self {
        Self {
            column,
            keyword: matched_patch,
        }
    }
}

/// Encapsulates all the matches inside a file
#[derive(Debug, Default, Getters)]
pub struct FileState {
    /// Container for organizing matches by row
    #[getset(get = "pub")]
    rows: HashMap<Row, (RowMetadata, Vec<Match>)>,
}

impl FileState {
    pub fn try_new(data: HashMap<Row, (RowMetadata, Vec<Match>)>) -> Option<Self> {
        (!data.is_empty()).then_some(Self { rows: data })
    }

    pub fn replace(&mut self, row: Row, metadata: RowMetadata, new_matches: Vec<Match>) {
        self.rows.insert(row, (metadata, new_matches));
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
