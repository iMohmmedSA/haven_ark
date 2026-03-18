mod app;
mod component;
mod constants;
mod theme;
mod views;

use iced::window::Settings;

use crate::app::App;

fn main() -> iced::Result {
    let mut window_settings = Settings::default();
    window_settings.icon = Some(load_icon());

    iced::application(App::new, App::update, App::view)
        .theme(App::theme())
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
