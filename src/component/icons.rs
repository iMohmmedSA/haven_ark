/// Generated automatically by build.rs
/// Do not edit manually.
/// Icon hash (SHA-256): C62092FF06FA1157B054E106000F454A98CF169DB15009DD791CF73EF63F2F10
use iced::{Font, widget::Text};

pub const FONT: &[u8] = include_bytes!("../../fonts/icons.ttf");

#[derive(Copy, Clone, Debug)]
pub enum Icons {
    RightArrow,
    Key,
    Logo,
}

impl Icons {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RightArrow => "\u{E000}",
            Self::Key => "\u{E001}",
            Self::Logo => "\u{E002}",
        }
    }

    #[inline]
    pub fn symbol(self) -> Text<'static> {
        Text::new(self.as_str()).font(Font::with_name("icons"))
    }
}
