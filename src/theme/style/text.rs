use iced::{Font, font, widget::Text};

pub trait TextExt {
    fn bold(self) -> Self;
}

impl TextExt for Text<'_> {
    fn bold(self) -> Self {
        self.font(Font {
            weight: font::Weight::Bold,
            ..Font::DEFAULT
        })
    }
}
