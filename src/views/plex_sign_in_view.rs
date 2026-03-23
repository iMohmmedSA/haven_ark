use iced::{
    Element, Length, Padding, Size, Task,
    alignment::{Horizontal, Vertical},
    font::Weight,
    widget::{
        self, Button, Column, Container, Sensor, Space, Text, center, column, qr_code, row, rule,
        space::horizontal, text::secondary,
    },
};
use plex_client::{
    client::{Client, crypto::DeviceKeypair},
    models::auth::pin::PinResponse,
};

use crate::{
    component::{Icons, gate_view},
    constants::GITHUB_URL,
    loadable::Loadable,
    theme::{
        style::{
            self, button,
            container::{card, card_primary},
            qr_code::transparent,
            text::TextExt,
        },
        token::{color, font_size, space},
    },
};

pub enum Action {
    Task(Task<Message>),
    None,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenGithub,
    OpenPlexSignIn,
    Loading(Size),
    RequestPin(Loadable<PinResponse>),
}

#[derive(Debug)]
pub struct PlexSignInView {
    client: Client,
    keypair: DeviceKeypair,
    pins: Loadable<PinResponse>,
    qr_code: Option<qr_code::Data>,
}

impl PlexSignInView {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            keypair: DeviceKeypair::generate(),
            pins: Loadable::Idle,
            qr_code: None,
        }
    }
}

impl PlexSignInView {
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::OpenGithub => self.handle_open_github(),
            Message::OpenPlexSignIn => self.handle_open_plex_sign_in(),
            Message::Loading(_) => self.handle_loading(),
            Message::RequestPin(pin_response) => self.handle_request_pin(pin_response),
        }
    }

    fn handle_open_github(&mut self) -> Action {
        let _ = open::that(GITHUB_URL);
        Action::None
    }

    fn handle_open_plex_sign_in(&mut self) -> Action {
        let url = match &self.pins {
            Loadable::Loaded(pin_response) => self.client.auth_url(&pin_response, None),
            _ => return Action::None,
        };
        let _ = open::that(url);
        Action::None
    }

    fn handle_loading(&mut self) -> Action {
        self.pins = Loadable::Loading;
        let client = self.client.clone();
        let keypair = self.keypair.clone();

        Action::Task(Task::perform(
            async move { client.request_pin(true, &keypair).await },
            |res| match res {
                Ok(pin_response) => Message::RequestPin(Loadable::Loaded(pin_response)),
                Err(e) => Message::RequestPin(Loadable::Error(e.to_string())),
            },
        ))
    }

    fn handle_request_pin(&mut self, pin_response: Loadable<PinResponse>) -> Action {
        self.pins = pin_response;
        let url = match &self.pins {
            Loadable::Loaded(pin_response) => self.client.auth_url(&pin_response, None),
            _ => return Action::None,
        };
        self.qr_code = qr_code::Data::new(&url).ok();
        Action::None
    }
}

impl PlexSignInView {
    pub fn view(&self) -> Element<'_, Message> {
        let content: Element<_> = match &self.pins {
            Loadable::Idle => Sensor::new("..").on_show(Message::Loading).into(),
            Loadable::Loading => "Loading...".into(),
            Loadable::Loaded(_) => Container::new(row![self.left_side(), self.right_side()])
                .padding(space::XXL)
                .style(card)
                .into(),
            Loadable::Error(e) => Text::new(e).into(),
        };

        gate_view(content).into()
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
                Icons::RightArrow.symbol().size(font_size::XL),
                horizontal(),
            ]
            .width(Length::Fill)
            .align_y(Vertical::Center)
            .spacing(space::SM))
            .on_press(Message::OpenPlexSignIn)
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
            .on_press(Message::OpenGithub)
            .style(button::outline)
            .padding(Padding::ZERO.vertical(space::MD)),

            Space::new().height(space::XXL),

            row![
                horizontal(),
                Button::new(Text::new("GitHub").size(font_size::MD))
                    .on_press(Message::OpenGithub)
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
            self.qr_code(),
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

    fn qr_code(&self) -> Container<'_, Message> {
        let content = self
            .qr_code
            .as_ref()
            .map(|qr| widget::qr_code(qr).style(transparent));
        let content: Element<_> = match content {
            Some(content) => content.into(),
            None => Text::new("Shouldn't show").into(),
        };

        center(content).height(224).width(224).style(card_primary)
    }
}
