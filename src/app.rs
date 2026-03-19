use std::sync::Arc;

use iced::{
    Element, Task, Theme,
    theme::{Custom, Palette},
};

use crate::{
    constants::APP_NAME,
    theme::token::color,
    views::plex_sign_in_view::{self, PlexSignInView},
};

#[derive(Debug)]
enum View {
    PlexSignIn(PlexSignInView),
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    PlexSignIn(plex_sign_in_view::Message),
}

#[derive(Debug)]
pub struct App {
    view: View,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                view: View::PlexSignIn(PlexSignInView::new()),
            },
            Task::none(),
        )
    }

    pub fn theme(&self) -> Theme {
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

    pub fn title(&self) -> String {
        APP_NAME.to_string()
    }
}

impl App {
    pub fn update(&mut self, message: Message) {}
}

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        match &self.view {
            View::PlexSignIn(view) => view.view().map(Message::PlexSignIn),
        }
        .into()
    }
}
