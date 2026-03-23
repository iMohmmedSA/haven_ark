use iced::{Color, Theme, widget::qr_code::Style};

pub fn transparent(theme: &Theme) -> Style {
    let palette = theme.palette();

    Style {
        cell: palette.background,
        background: Color::TRANSPARENT,
    }
}
