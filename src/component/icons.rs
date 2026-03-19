/// Generated automatically by build.rs
/// Do not edit manually.
/// Icon hash (SHA-256): 14F2027170D24B5EB31CA32C11D7FC04FE9C790BCF6A25F80F7F15EB6337D146
use iced::{Font, widget::Text};

pub const FONT: &[u8] = include_bytes!("../../fonts/icons.ttf");

#[derive(Copy, Clone, Debug)]
pub enum Icons {
    Right_arrow,
    Key,
    Logo,
}

impl Icons {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Right_arrow => "\u{E000}",
            Self::Key => "\u{E001}",
            Self::Logo => "\u{E002}",
        }
    }

    #[inline]
    pub fn symbol(self) -> Text<'static> {
        Text::new(self.as_str()).font(Font::with_name("icons"))
    }
}
