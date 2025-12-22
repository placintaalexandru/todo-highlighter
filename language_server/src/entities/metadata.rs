use getset::Getters;

use crate::entities::Column;

#[derive(Debug, Getters)]
pub struct RowMetadata {
    #[getset(get = "pub")]
    line_len: usize,
    #[getset(get = "pub")]
    first_non_zero_pos: Column,
}

impl RowMetadata {
    pub fn new(line_len: usize, first_non_zero_pos: Column) -> Self {
        Self {
            line_len,
            first_non_zero_pos,
        }
    }
}
