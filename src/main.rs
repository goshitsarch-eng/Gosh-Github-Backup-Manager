mod app;
mod types;
mod theme;
mod services;
mod pages;
mod widgets;

use app::GoshApp;
use iced::Font;

fn main() -> iced::Result {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    iced::application(GoshApp::title, GoshApp::update, GoshApp::view)
        .subscription(GoshApp::subscription)
        .theme(GoshApp::theme)
        .window_size((1280.0, 800.0))
        .font(include_bytes!("../assets/fonts/SpaceGrotesk.ttf").as_slice())
        .font(include_bytes!("../assets/fonts/Inter.ttf").as_slice())
        .font(include_bytes!("../assets/fonts/JetBrainsMono.ttf").as_slice())
        .default_font(Font::with_name("Inter"))
        .run_with(GoshApp::new)
}
