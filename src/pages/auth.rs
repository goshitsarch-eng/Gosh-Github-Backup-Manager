use crate::app::{GoshApp, Message};
use crate::types::{AuthMethod, OAuthStatus};
use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Element, Length};

impl GoshApp {
    pub fn view_auth(&self) -> Element<'_, Message> {
        let title = text("Gosh GitHub Backup Manager").size(28);
        let subtitle =
            text("Connect your GitHub account to backup your repositories").size(14);

        // Auth method toggle buttons
        let token_tab = {
            let mut btn = button(text("Personal Token").size(13)).padding([8, 20]);
            btn = if self.auth_method == AuthMethod::Token {
                btn.style(button::primary)
            } else {
                btn.style(button::secondary)
                    .on_press(Message::AuthMethodChanged(AuthMethod::Token))
            };
            btn
        };

        let oauth_tab = {
            let mut btn = button(text("Sign in with GitHub").size(13)).padding([8, 20]);
            btn = if self.auth_method == AuthMethod::OAuth {
                btn.style(button::primary)
            } else {
                btn.style(button::secondary)
                    .on_press(Message::AuthMethodChanged(AuthMethod::OAuth))
            };
            btn
        };

        let tabs = row![token_tab, oauth_tab].spacing(8);

        // Content based on selected auth method
        let method_content: Element<'_, Message> = match self.auth_method {
            AuthMethod::Token => self.view_auth_token(),
            AuthMethod::OAuth => self.view_auth_oauth(),
        };

        let card_content = column![
            title,
            subtitle,
            Space::with_height(16),
            tabs,
            Space::with_height(16),
            method_content,
        ]
        .spacing(8)
        .align_x(Alignment::Center)
        .max_width(420);

        container(
            container(card_content)
                .padding(32)
                .style(container::bordered_box),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }

    fn view_auth_token(&self) -> Element<'_, Message> {
        let token_label = text("Personal Access Token").size(13);
        let token_input = text_input("ghp_xxxxxxxxxxxx", &self.token_input)
            .on_input(Message::TokenInputChanged)
            .on_submit(Message::LoginSubmit)
            .padding(10)
            .size(14)
            .secure(true);

        let mut connect_btn = button(
            text(if self.is_loading {
                "Connecting..."
            } else {
                "Connect to GitHub"
            })
            .size(14),
        )
        .padding([10, 24])
        .width(Length::Fill)
        .style(button::primary);

        if !self.is_loading && !self.token_input.is_empty() {
            connect_btn = connect_btn.on_press(Message::LoginSubmit);
        }

        let error_text = if let Some(ref status) = self.auth_status {
            text(status.as_str())
                .size(12)
                .color(iced::Color::from_rgb(0.9, 0.3, 0.3))
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

        let create_token_btn = button(text("Create a new token on GitHub").size(12))
            .padding([6, 12])
            .style(button::secondary)
            .on_press(Message::OpenUrl(
                "https://github.com/settings/tokens/new?scopes=repo,read:user,read:org&description=Gosh%20GitHub%20Backup%20Manager".to_string(),
            ));

        column![
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
        .into()
    }

    fn view_auth_oauth(&self) -> Element<'_, Message> {
        match &self.oauth_status {
            OAuthStatus::Idle => {
                let description = text(
                    "Sign in through your browser. No token needed.",
                )
                .size(13);

                let sign_in_btn = button(text("Sign in with GitHub").size(14))
                    .padding([10, 24])
                    .width(Length::Fill)
                    .style(button::primary)
                    .on_press(Message::OAuthStartDeviceFlow);

                column![description, Space::with_height(8), sign_in_btn]
                    .spacing(8)
                    .into()
            }
            OAuthStatus::RequestingCode => {
                let loading = text("Connecting to GitHub...").size(14);
                column![loading].spacing(8).into()
            }
            OAuthStatus::WaitingForUser {
                user_code,
                verification_uri,
                ..
            } => {
                let instruction =
                    text("Enter this code on GitHub:").size(13);

                let code_display = text(user_code.as_str()).size(32);

                let open_btn =
                    button(text("Open GitHub").size(14))
                        .padding([10, 24])
                        .width(Length::Fill)
                        .style(button::primary)
                        .on_press(Message::OpenUrl(verification_uri.clone()));

                let waiting = text("Waiting for authorization...")
                    .size(12)
                    .color(iced::Color::from_rgb(0.5, 0.5, 0.5));

                let cancel_btn = button(text("Cancel").size(12))
                    .padding([6, 12])
                    .style(button::secondary)
                    .on_press(Message::OAuthCancel);

                column![
                    instruction,
                    Space::with_height(8),
                    code_display,
                    Space::with_height(12),
                    open_btn,
                    Space::with_height(8),
                    waiting,
                    Space::with_height(8),
                    cancel_btn,
                ]
                .spacing(4)
                .align_x(Alignment::Center)
                .into()
            }
            OAuthStatus::Error(msg) => {
                let error = text(msg.as_str())
                    .size(13)
                    .color(iced::Color::from_rgb(0.9, 0.3, 0.3));

                let retry_btn = button(text("Try Again").size(14))
                    .padding([10, 24])
                    .width(Length::Fill)
                    .style(button::primary)
                    .on_press(Message::OAuthStartDeviceFlow);

                column![error, Space::with_height(8), retry_btn]
                    .spacing(8)
                    .into()
            }
        }
    }
}
