mod app;
mod app_data;
mod component;
mod constants;
mod error;
mod theme;
mod views;

use iced::window::Settings;

use crate::{app::App, component::icons};

fn main() -> iced::Result {
    let mut window_settings = Settings::default();
    window_settings.icon = Some(load_icon());

    iced::application(App::new, App::update, App::view)
        .title(App::title)
        .theme(App::theme)
        .font(icons::FONT)
        .window(window_settings)
        .run()
}

fn load_icon() -> iced::window::Icon {
    let bytes = include_bytes!("../assets/icon.png");
    let img = image::load_from_memory(bytes).unwrap().into_rgba8();
    let (width, height) = img.dimensions();
    let rgba = img.into_raw();
    iced::window::icon::from_rgba(rgba, width, height).unwrap()
}
