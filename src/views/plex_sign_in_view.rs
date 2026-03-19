use iced::{
    Element, Length, Padding,
    alignment::{Horizontal, Vertical},
    font::Weight,
    widget::{Button, Column, Space, Text, column, row, rule, space::horizontal, text::secondary},
};

use crate::{
    component::{Icons, gate_view},
    constants::GITHUB_URL,
    theme::{
        style::{self, button, text::TextExt},
        token::{color, font_size, space},
    },
};

pub enum Action {
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    OpenLink,
}

#[derive(Debug)]
pub struct PlexSignInView;

impl PlexSignInView {
    pub fn new() -> Self {
        Self
    }
}

impl PlexSignInView {
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::OpenLink => self.handle_open_link(),
        }
    }

    fn handle_open_link(&mut self) -> Action {
        let _ = open::that(GITHUB_URL);
        Action::None
    }
}

impl PlexSignInView {
    pub fn view(&self) -> Element<'_, Message> {
        gate_view(row![self.left_side(), self.right_side()]).into()
    }

    fn right_side(&self) -> Column<'_, Message> {
        column![
            Text::new("Sign in to Plex").weight(Weight::Bold).size(font_size::XXL),
            Space::new().height(space::MD),
            Text::new("Connect your Plex account and get instant access to your entire media library, ready to enjoy.").style(secondary).size(font_size::LG),
            Space::new().height(space::XXL),

            Button::new(row![
                horizontal(),
                Text::new("Sign In Via Plex.tv").size(font_size::MD).weight(Weight::Bold),
                Icons::Right_arrow.symbol().size(font_size::XL),
                horizontal(),
            ]
            .width(Length::Fill)
            .align_y(Vertical::Center)
            .spacing(space::SM))
            .on_press(Message::OpenLink)
            .style(button::primary)
            .padding(Padding::ZERO.vertical(space::MD)),

            row![
                rule::horizontal(1).style(style::rule::secondary),
                Text::new("OR").weight(Weight::Medium).style(secondary),
                rule::horizontal(1).style(style::rule::secondary)
            ]
            .align_y(Vertical::Center)
            .spacing(space::MD)
            .padding(Padding::ZERO.vertical(space::MD)),

            Button::new(row![
                horizontal(),
                Icons::Key.symbol().size(font_size::XL),
                Text::new("Link with 4-Digit Code").color(color::TEXT).size(font_size::MD).weight(Weight::Bold),
                horizontal(),
            ]
            .width(Length::Fill)
            .align_y(Vertical::Center)
            .spacing(space::SM))
            .on_press(Message::OpenLink)
            .style(button::outline)
            .padding(Padding::ZERO.vertical(space::MD)),

            Space::new().height(space::XXL),

            row![
                horizontal(),
                Button::new(Text::new("GitHub").size(font_size::MD))
                    .on_press(Message::OpenLink)
                    .style(button::secondary_text)
                    .padding(Padding::ZERO.vertical(space::MD)),
                horizontal()
            ],
        ]
        .padding(Padding::ZERO.left(space::XXL))
        .width(450)
    }

    fn left_side(&self) -> Column<'_, Message> {
        column![
            Button::new(Space::new().height(224).width(224)),
            Space::new().height(space::XXL),
            Text::new("Scan to Login")
                .weight(Weight::Bold)
                .size(font_size::LG),
            Space::new().height(space::SM),
            Text::new("Open your camera app to sign in instantly.")
                .style(secondary)
                .size(font_size::MD),
        ]
        .align_x(Horizontal::Center)
        .padding(Padding::ZERO.right(space::XXL))
        .width(450)
    }
}
