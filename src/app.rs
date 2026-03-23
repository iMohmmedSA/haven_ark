use std::sync::Arc;

use iced::{
    Element, Task, Theme,
    theme::{Custom, Palette},
    widget::Text,
};
use plex_client::client::Client;
use sysinfo::System;

use crate::{
    app_data::AppData,
    constants::{APP_NAME, APP_VERSION},
    theme::token::color,
    views::plex_sign_in_view::{self, PlexSignInView},
};

#[derive(Debug)]
enum View {
    PlexSignIn(PlexSignInView),
    Tmp,
}

#[derive(Debug, Clone)]
pub enum Message {
    PlexSignIn(plex_sign_in_view::Message),
}

#[derive(Debug)]
pub struct AppState {
    plex_client: Client,
}

impl AppState {
    pub fn new(client_id: &str) -> Self {
        let plex_client = Client::builder()
            .client_identifier(client_id)
            // .token(token)
            .product(APP_NAME)
            .version(APP_VERSION)
            .platform(System::name().unwrap_or_default())
            .platform_version(System::os_version().unwrap_or_default())
            .build()
            .unwrap_or_else(|e| panic!("Failed to build client, this is a bug: {e}"));

        Self { plex_client }
    }
}

#[derive(Debug)]
pub struct App {
    state: AppState,
    data: AppData,
    view: View,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let Ok(data) = AppData::load() else {
            panic!("Failed to load app data")
        };
        let state = AppState::new(&data.plex.client_id);
        let client = state.plex_client.clone();

        (
            Self {
                state,
                data,
                view: View::PlexSignIn(PlexSignInView::new(client)),
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
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PlexSignIn(msg) => self.handle_plex_sign_in(msg),
        }
    }

    fn handle_plex_sign_in(&mut self, message: plex_sign_in_view::Message) -> Task<Message> {
        let View::PlexSignIn(view) = &mut self.view else {
            return Task::none();
        };

        use plex_sign_in_view::Action;
        match view.update(message) {
            Action::Task(task) => task.map(Message::PlexSignIn),
            Action::None => Task::none(),
        }
    }
}

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        match &self.view {
            View::PlexSignIn(view) => view.view().map(Message::PlexSignIn).into(),
            View::Tmp => Text::new("fragment").into(),
        }
    }
}
