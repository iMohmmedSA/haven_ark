use iced::{Font, font, widget::Text};

pub trait TextExt {
    fn weight(self, weight: font::Weight) -> Self;
}

impl TextExt for Text<'_> {
    fn weight(self, weight: font::Weight) -> Self {
        self.font(Font {
            weight,
            ..Font::DEFAULT
        })
    }
}
