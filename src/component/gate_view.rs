use iced::{
    Element,
    alignment::{Horizontal, Vertical},
    font::Weight,
    widget::{Column, Text, center, column, container, row, text::secondary},
};

use crate::{
    component::Icons,
    constants::{APP_NAME, APP_VERSION},
    theme::{
        style::text::TextExt,
        token::{font_size, space},
    },
};

pub fn gate_view<'a, M: 'a>(content: impl Into<Element<'a, M>>) -> Column<'a, M> {
    let header = row![
        // FIX: there is a spacing issue in the logo
        Icons::Logo.symbol().size(font_size::DISPLAY),
        Text::new(APP_NAME).weight(Weight::Bold).size(font_size::XL)
    ]
    .align_y(Vertical::Center)
    .padding(space::XL)
    .spacing(space::MD);
    let footer = container(
        Text::new(format!("{} v{}", APP_NAME, APP_VERSION))
            .size(font_size::SM)
            .style(secondary),
    )
    .padding(space::XXL);

    column![header, center(content), footer].align_x(Horizontal::Center)
}
