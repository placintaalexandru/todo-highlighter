use crate::{
    entities::{Color, ColorType, Colors, Column, Match, RowMetadata},
    use_cases::ports::Colorer,
};

pub struct Highlight<T> {
    inner: T,
}

impl<T> Highlight<T> {
    pub fn new(v: T) -> Self {
        Self { inner: v }
    }
}

impl<T: Colorer> Highlight<T> {
    pub fn highlight(&self, text: &str, color_type: ColorType) -> Option<Color> {
        self.inner.color_text(text, color_type)
    }

    pub fn update_palette(&mut self, key: String, value: Colors) {
        self.inner.update_palette(key, value);
    }

    pub fn color_intervals(
        &self,
        row_match_beginnings: &[Match],
        row_meta: &RowMetadata,
    ) -> Vec<(Column, Column)> {
        self.inner.color_intervals(row_match_beginnings, row_meta)
    }
}
