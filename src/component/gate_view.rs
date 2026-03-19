use iced::{
    Element,
    alignment::{Horizontal, Vertical},
    font::Weight,
    widget::{Column, Container, Text, center, column, container, row, text::secondary},
};

use crate::{
    component::Icons,
    constants::{APP_NAME, APP_VERSION},
    theme::{
        style::{container::card, text::TextExt},
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

    column![header, body(content), footer].align_x(Horizontal::Center)
}

fn body<'a, M: 'a>(content: impl Into<Element<'a, M>>) -> Container<'a, M> {
    center(Container::new(content).padding(space::XXL).style(card))
}
