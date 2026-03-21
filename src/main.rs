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

    let icon = iced::window::icon::from_rgba(
        include_bytes!("../assets/icon_32x32.rgba").to_vec(),
        32,
        32,
    )
    .ok();

    let mut app = iced::application(GoshApp::title, GoshApp::update, GoshApp::view)
        .subscription(GoshApp::subscription)
        .theme(GoshApp::theme)
        .window_size((1280.0, 800.0))
        .font(include_bytes!("../assets/fonts/SpaceGrotesk.ttf").as_slice())
        .font(include_bytes!("../assets/fonts/Inter.ttf").as_slice())
        .font(include_bytes!("../assets/fonts/JetBrainsMono.ttf").as_slice())
        .default_font(Font::with_name("Inter"));

    if let Some(icon) = icon {
        app = app.window(iced::window::Settings {
            icon: Some(icon),
            ..Default::default()
        });
    }

    app.run_with(GoshApp::new)
}
