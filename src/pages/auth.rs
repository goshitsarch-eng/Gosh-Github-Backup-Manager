use crate::app::{GoshApp, Message};
use iced::widget::{button, column, container, text, text_input, Space};
use iced::{Alignment, Element, Length};

impl GoshApp {
    pub fn view_auth(&self) -> Element<'_, Message> {
        let title = text("Gosh GitHub Backup Manager").size(28);
        let subtitle = text("Connect your GitHub account to backup your repositories").size(14);

        let token_label = text("Personal Access Token").size(13);
        let token_input = text_input("ghp_xxxxxxxxxxxx", &self.token_input)
            .on_input(Message::TokenInputChanged)
            .on_submit(Message::LoginSubmit)
            .padding(10)
            .size(14)
            .secure(true);

        let mut connect_btn = button(
            text(if self.is_loading { "Connecting..." } else { "Connect to GitHub" }).size(14)
        )
        .padding([10, 24])
        .width(Length::Fill)
        .style(button::primary);

        if !self.is_loading && !self.token_input.is_empty() {
            connect_btn = connect_btn.on_press(Message::LoginSubmit);
        }

        let error_text = if let Some(ref status) = self.auth_status {
            text(status.as_str()).size(12).color(iced::Color::from_rgb(0.9, 0.3, 0.3))
        } else {
            text("").size(12)
        };

        let scopes_info = column![
            text("Required token scopes:").size(11),
            text("  \u{2022} repo - Access repositories").size(11),
            text("  \u{2022} read:user - Read user profile").size(11),
            text("  \u{2022} read:org - Read organizations").size(11),
        ]
        .spacing(4);

        let create_token_btn = button(
            text("Create a new token on GitHub").size(12)
        )
        .padding([6, 12])
        .style(button::secondary)
        .on_press(Message::OpenUrl("https://github.com/settings/tokens/new?scopes=repo,read:user,read:org&description=Gosh%20GitHub%20Backup%20Manager".to_string()));

        let card_content = column![
            title,
            subtitle,
            Space::with_height(20),
            token_label,
            token_input,
            error_text,
            Space::with_height(8),
            connect_btn,
            Space::with_height(16),
            create_token_btn,
            Space::with_height(16),
            scopes_info,
        ]
        .spacing(8)
        .align_x(Alignment::Center)
        .max_width(420);

        container(
            container(card_content)
                .padding(32)
                .style(container::bordered_box)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }
}
