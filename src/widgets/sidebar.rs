use crate::app::Message;
use crate::theme;
use crate::types::{GitHubUser, Page};
use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Element, Length};

fn nav_item(label: &str, page: Page, current: Page, is_locked: bool) -> Element<'static, Message> {
    let is_active = page == current;

    if is_locked {
        // Locked nav item
        let content = row![
            text(label.to_string())
                .size(13)
                .color(iced::Color { a: 0.35, ..theme::colors::ON_SURFACE_VARIANT }),
            Space::with_width(Length::Fill),
            text("\u{1F512}")
                .size(10)
                .color(iced::Color { a: 0.3, ..theme::colors::ON_SURFACE_VARIANT }),
        ]
        .align_y(Alignment::Center)
        .padding([0, 16]);

        return container(content)
            .width(Length::Fill)
            .padding([10, 8])
            .into();
    }

    if is_active {
        // Active nav item with tertiary accent bar
        let accent = container(Space::new(3, Length::Fill))
            .style(theme::accent_bar(theme::colors::TERTIARY));

        let btn_content = button(
            text(label.to_string())
                .size(13)
                .color(theme::colors::PRIMARY)
        )
        .width(Length::Fill)
        .padding([10, 16])
        .style(theme::nav_active)
        .on_press(Message::NavigateTo(page));

        row![accent, btn_content]
            .height(Length::Shrink)
            .into()
    } else {
        // Inactive nav item
        button(
            text(label.to_string())
                .size(13)
        )
        .width(Length::Fill)
        .padding([10, 20])
        .style(theme::nav_inactive)
        .on_press(Message::NavigateTo(page))
        .into()
    }
}

pub fn view(current_page: Page, is_authenticated: bool, user: Option<&GitHubUser>) -> Element<'static, Message> {
    // Branding
    let branding = column![
        text("GitHub Backup")
            .size(18)
            .font(theme::FONT_HEADLINE)
            .color(theme::colors::PRIMARY),
        text(if is_authenticated { "V2.0.0 \u{00B7} Connected" } else { "V2.0.0 \u{00B7} Not Connected" })
            .size(10)
            .font(theme::FONT_MONO)
            .color(theme::colors::OUTLINE),
    ]
    .spacing(4);

    let branding_section = container(branding)
        .padding([24, 24]);

    // Navigation
    let nav = column![
        nav_item("Dashboard", Page::Dashboard, current_page, !is_authenticated),
        nav_item("Repositories", Page::Repositories, current_page, !is_authenticated),
        nav_item("Backup", Page::Backup, current_page, !is_authenticated),
        nav_item("Settings", Page::Settings, current_page, false),
    ]
    .spacing(2);

    // Bottom section
    let docs_btn = button(
        text("Docs")
            .size(12)
    )
    .width(Length::Fill)
    .padding([8, 20])
    .style(theme::ghost_button)
    .on_press(Message::OpenUrl("https://github.com/goshitsarch-eng/Gosh-Github-Backup-Manager".to_string()));

    let logout_btn = button(
        text("Logout")
            .size(12)
    )
    .width(Length::Fill)
    .padding([8, 20])
    .style(theme::ghost_button)
    .on_press(Message::LogoutRequested);

    let mut bottom = column![docs_btn].spacing(2);
    if is_authenticated {
        bottom = bottom.push(logout_btn);
    }

    // User profile card at bottom (if authenticated)
    let user_section: Element<'static, Message> = if let Some(user) = user {
        let login = user.login.clone();
        let initials = user
            .name
            .as_deref()
            .or(Some(&user.login))
            .unwrap_or("?")
            .chars()
            .next()
            .unwrap_or('?')
            .to_uppercase()
            .to_string();

        container(
            row![
                container(
                    text(initials)
                        .size(13)
                        .color(theme::colors::ON_SURFACE)
                )
                .width(32)
                .height(32)
                .center_x(32)
                .center_y(32)
                .style(theme::card_highest),
                column![
                    text(login)
                        .size(12)
                        .color(theme::colors::ON_SURFACE),
                ]
                .spacing(2),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
        )
        .padding([12, 20])
        .width(Length::Fill)
        .style(|_theme: &iced::Theme| iced::widget::container::Style {
            border: iced::Border {
                color: theme::colors::WHITE_5,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
    } else {
        Space::with_height(0).into()
    };

    container(
        column![
            branding_section,
            nav,
            Space::with_height(Length::Fill),
            bottom,
            user_section,
        ]
    )
    .width(264)
    .height(Length::Fill)
    .style(theme::sidebar)
    .into()
}
