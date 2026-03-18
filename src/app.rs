use std::sync::Arc;

use iced::{
    Task, Theme,
    theme::{Custom, Palette},
    widget::{Button, Column, Text},
};

use crate::{component::gate_view, theme::token::color};

#[derive(Debug, Clone, Copy)]
pub enum Message {}

#[derive(Default, Debug)]
pub struct App;

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (Self, Task::none())
    }

    pub fn theme() -> Theme {
        let palette = Palette {
            background: color::BG,
            text: color::TEXT,
            primary: color::PRIMARY,
            success: color::SUCCESS,
            warning: color::WARNING,
            danger: color::DANGER,
        };
        let theme = Custom::new("GrayScale".into(), palette);

        Theme::Custom(Arc::new(theme))
    }
}

impl App {
    pub fn update(&mut self, message: Message) {}
}

impl App {
    pub fn view(&self) -> Column<'_, Message> {
        gate_view(Text::new("Test"))
    }
}
