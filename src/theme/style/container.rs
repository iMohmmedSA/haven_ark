use iced::{Border, Theme, widget::container::Style};

use crate::theme::token::radius;

pub fn card(theme: &Theme) -> Style {
    let background = theme.extended_palette().background.strongest;
    Style {
        background: Some(background.color.into()),
        border: Border::default().rounded(radius::LG), // Should be 16 for sgin in
        ..Default::default()
    }
}
