use iced::{
    Element, Length,
    alignment::{Horizontal, Vertical},
    widget::{Column, Container, Text, column, container, row, space::vertical, text::secondary},
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
        Text::new(APP_NAME).bold().size(font_size::XL)
    ]
    .align_y(Vertical::Center)
    .padding(space::XL)
    .spacing(space::MD);
    let footer = container(
        Text::new(format!("{} v{}", APP_NAME, APP_VERSION))
            .size(font_size::SM)
            .style(secondary),
    )
    .padding(space::XL);

    column![header, vertical(), body(content), vertical(), footer].align_x(Horizontal::Center)
}

fn body<'a, M: 'a>(content: impl Into<Element<'a, M>>) -> Container<'a, M> {
    Container::new(Container::new(content).padding(20).style(card)).center(Length::Fill)
}
