use crate::app::Message;
use crate::types::Page;
use iced::widget::{button, column, container, text, Space};
use iced::{Alignment, Element, Length};

fn nav_button(label: &str, icon: &str, page: Page, current: Page) -> Element<'static, Message> {
    let is_active = page == current;
    let label_text = format!("{} {}", icon, label);

    let btn = button(
        text(label_text)
            .size(14)
    )
    .width(Length::Fill)
    .padding([10, 16])
    .on_press(Message::NavigateTo(page));

    let btn = if is_active {
        btn.style(button::primary)
    } else {
        btn.style(button::secondary)
    };

    btn.into()
}

pub fn view(current_page: Page) -> Element<'static, Message> {
    let header = container(
        column![
            text("Gosh").size(20),
            text("GitHub Backup Manager").size(11),
        ]
        .align_x(Alignment::Center)
    )
    .padding([20, 16])
    .width(Length::Fill);

    let nav = column![
        nav_button("Dashboard", "\u{1F4CA}", Page::Dashboard, current_page),
        nav_button("Repositories", "\u{1F4C1}", Page::Repositories, current_page),
        nav_button("Backup", "\u{1F4BE}", Page::Backup, current_page),
        nav_button("Settings", "\u{2699}", Page::Settings, current_page),
        nav_button("About", "\u{2139}", Page::About, current_page),
    ]
    .spacing(4)
    .padding([8, 8]);

    let version = container(
        text("v2.0.0").size(11)
    )
    .padding([16, 16]);

    container(
        column![
            header,
            nav,
            Space::with_height(Length::Fill),
            version,
        ]
    )
    .width(220)
    .height(Length::Fill)
    .style(container::dark)
    .into()
}
