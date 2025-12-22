use std::collections::HashMap;

use crate::{
    adapters::config::UserColors,
    entities::{Color, ColorType, Colors, Column, Match, RowMetadata},
    use_cases::{Highlight as UseCase, ports::Colorer},
};

pub struct Highlight<T> {
    inner: UseCase<T>,
}

impl<T> Highlight<T> {
    pub fn new(v: T) -> Self {
        Self {
            inner: UseCase::new(v),
        }
    }
}

impl<T: Colorer> Highlight<T> {
    pub fn highlight(&self, text: &str, color_type: ColorType) -> Option<Color> {
        self.inner.highlight(text, color_type)
    }

    pub fn color_intervals(
        &self,
        row_matches: &[Match],
        row_meta: &RowMetadata,
    ) -> Vec<(Column, Column)> {
        self.inner.color_intervals(row_matches, row_meta)
    }

    pub fn update_palette(&mut self, palette: HashMap<String, UserColors>) {
        palette.into_iter().for_each(|(key, user_colors)| {
            let UserColors { background } = user_colors;
            let colors = Colors::builder()
                .background(Color::new(
                    background.r,
                    background.g,
                    background.b,
                    background.a,
                ))
                .build();
            self.inner.update_palette(key, colors);
        });
    }
}
