use iced::{
    Theme,
    widget::rule::{FillMode, Style},
};

pub fn secondary(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        color: palette.primary.weak.text,
        radius: 0.0.into(),
        fill_mode: FillMode::Full,
        snap: true,
    }
}
