use iced::{
    Background, Shadow, Theme, border,
    widget::button::{Status, Style},
};

use crate::theme::token::radius;

pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = Style {
        background: Some(Background::Color(palette.primary.base.color)),
        text_color: palette.primary.base.text,
        border: border::rounded(radius::MD),
        ..Style::default()
    };

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            shadow: Shadow {
                color: palette.primary.base.color,
                blur_radius: 5.0,
                ..Default::default()
            },
            ..base
        },
        Status::Disabled => Style {
            background: base
                .background
                .map(|background| background.scale_alpha(0.5)),
            text_color: base.text_color.scale_alpha(0.5),
            ..base
        },
    }
}

pub fn outline(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = Style {
        background: None,
        text_color: palette.secondary.weak.color,
        border: border::rounded(radius::MD)
            .color(palette.primary.weak.text)
            .width(1),
        ..Style::default()
    };

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: None,
            text_color: palette.primary.base.color,
            border: border::rounded(radius::MD)
                .color(palette.primary.base.text)
                .width(1),
            ..base
        },
        Status::Disabled => Style {
            text_color: base.text_color.scale_alpha(0.5),
            ..base
        },
    }
}

pub fn secondary_text(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let base = Style {
        text_color: palette.secondary.base.color,
        ..Style::default()
    };

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            text_color: palette.secondary.base.color.scale_alpha(0.8),
            ..base
        },
        Status::Disabled => Style {
            text_color: base.text_color.scale_alpha(0.5),
            ..base
        },
    }
}
