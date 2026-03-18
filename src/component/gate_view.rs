use iced::{
    Element, Length,
    alignment::Horizontal,
    widget::{Column, Container, Text, column, container, row, space::vertical, text::secondary},
};

use crate::{
    constants::{APP_NAME, APP_VERSION},
    theme::{
        style::container::card,
        token::{color, space},
    },
};

pub fn gate_view<'a, M: 'a>(content: impl Into<Element<'a, M>>) -> Column<'a, M> {
    let header = row![Text::new("ICON HERE"), Text::new(APP_NAME)]
        .padding(space::XL)
        .spacing(space::MD);
    let footer = container(Text::new(format!("{} v{}", APP_NAME, APP_VERSION)).style(secondary))
        .padding(space::XL);

    column![header, vertical(), body(content), vertical(), footer].align_x(Horizontal::Center)
}

fn body<'a, M: 'a>(content: impl Into<Element<'a, M>>) -> Container<'a, M> {
    Container::new(Container::new(content).padding(20).style(card)).center(Length::Fill)
}
