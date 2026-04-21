use std::sync::Arc;

use iced::{
    Element, Subscription, Task, Theme,
    theme::{Custom, Palette},
    widget::Text,
};
use plex_client::client::Client;
use sysinfo::System;

use crate::{
    app_data::{AppData, SecureStorage},
    constants::{APP_NAME, APP_VERSION},
    theme::token::color,
    views::{
        fatal_view::fatal_view,
        plex_sign_in_view::{self, PlexSignInView},
    },
};

#[derive(Debug)]
enum View {
    PlexSignIn(PlexSignInView),
    Fatal(String),
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
            .maybe_token(SecureStorage::PlexToken.get().ok())
            .product(APP_NAME)
            .version(APP_VERSION)
            .maybe_platform(System::name())
            .maybe_platform_version(System::os_version())
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

    pub fn subscription(&self) -> Subscription<Message> {
        match &self.view {
            View::PlexSignIn(view) => view.subscription().map(Message::PlexSignIn),
            View::Fatal(_) => Subscription::none(),
            View::Tmp => Subscription::none(),
        }
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
            Action::UpdatePlexToken(_) => todo!(),
            Action::Fatal(msg) => {
                self.view = View::Fatal(msg);
                Task::none()
            }
            Action::Task(task) => task.map(Message::PlexSignIn),
            Action::None => Task::none(),
        }
    }
}

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        match &self.view {
            View::PlexSignIn(view) => view.view().map(Message::PlexSignIn).into(),
            View::Fatal(msg) => fatal_view(msg).into(),
            View::Tmp => Text::new("fragment").into(),
        }
    }
}
