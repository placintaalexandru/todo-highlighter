use tower_lsp_server::ls_types;

use crate::{entities, use_cases::ports::Conversion};

pub struct Position;

impl Conversion for Position {
    type From = entities::Position;
    type To = ls_types::Position;

    fn convert(from: Self::From) -> Self::To {
        let (row, column) = from.into_pair();

        Self::To {
            line: row.row() as u32,
            character: column.column() as u32,
        }
    }
}
