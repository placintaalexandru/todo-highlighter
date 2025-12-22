use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ColorType {
    Background,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn into_components(self) -> (u8, u8, u8, u8) {
        let Self { r, g, b, a } = self;
        (r, g, b, a)
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 134,
            g: 134,
            b: 134,
            a: 255,
        }
    }
}

#[derive(Debug, Default, TypedBuilder)]
pub struct Colors {
    #[builder(default)]
    pub background: Color,
}

impl Colors {
    pub fn new(background: Color) -> Self {
        Self { background }
    }
}
