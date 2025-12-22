use tower_lsp_server::ls_types;

use crate::{entities, use_cases::ports::Conversion};

pub struct Color;

impl Conversion for Color {
    type From = entities::Color;
    type To = ls_types::Color;

    fn convert(from: Self::From) -> Self::To {
        let (r, g, b, a) = from.into_components();

        Self::To {
            red: f32::from(r) / 255.,
            green: f32::from(g) / 255.,
            blue: f32::from(b) / 255.,
            alpha: f32::from(a) / 255.,
        }
    }
}
