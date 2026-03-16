mod app;
mod types;
mod theme;
mod services;
mod pages;
mod widgets;

use app::GoshApp;

fn main() -> iced::Result {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    iced::application(GoshApp::title, GoshApp::update, GoshApp::view)
        .subscription(GoshApp::subscription)
        .theme(GoshApp::theme)
        .window_size((1280.0, 800.0))
        .run_with(GoshApp::new)
}
