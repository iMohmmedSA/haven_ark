use iced::widget::{Column, Text};

use crate::{app::Message, component::gate_view};

pub fn fatal_view<'a>(error: &'a str) -> Column<'a, Message> {
    gate_view(Text::new(error))
}
