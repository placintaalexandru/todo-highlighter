use getset::Getters;

use crate::entities::{Column, Row};

#[derive(Debug, Getters)]
pub struct Position {
    #[getset(get = "pub")]
    row: Row,
    #[getset(get = "pub")]
    column: Column,
}

impl Position {
    pub fn new(row: Row, column: Column) -> Self {
        Self { row, column }
    }

    pub fn into_pair(self) -> (Row, Column) {
        let Self { row, column } = self;
        (row, column)
    }
}
