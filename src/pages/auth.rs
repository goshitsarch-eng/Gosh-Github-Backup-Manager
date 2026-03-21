use crate::app::{GoshApp, Message};
use crate::theme;
use crate::types::{AuthMethod, OAuthStatus};
use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Element, Length};

impl GoshApp {
    pub fn view_auth(&self) -> Element<'_, Message> {
        let accent = container(Space::new(3, Length::Fill))
            .style(theme::accent_bar(theme::colors::TERTIARY));

        let title = text("Connect to GitHub")
            .size(28)
            .font(theme::FONT_HEADLINE);

        let subtitle = text("Link your account to enable automated backups and repository synchronization.")
            .size(13)
            .color(theme::colors::ON_SURFACE_VARIANT);

        // Auth method tabs (segmented control)
        let token_tab = {
            let btn = button(
                text("Personal Access Token")
                    .size(11)
                    .font(theme::FONT_HEADLINE)
            )
            .padding([10, 0])
            .width(Length::Fill);
            if self.auth_method == AuthMethod::Token {
                btn.style(theme::tab_active)
            } else {
                btn.style(theme::tab_inactive)
                    .on_press(Message::AuthMethodChanged(AuthMethod::Token))
            }
        };

        let oauth_tab = {
            let btn = button(
                text("Sign in with GitHub")
                    .size(11)
                    .font(theme::FONT_HEADLINE)
            )
            .padding([10, 0])
            .width(Length::Fill);
            if self.auth_method == AuthMethod::OAuth {
                btn.style(theme::tab_active)
            } else {
                btn.style(theme::tab_inactive)
                    .on_press(Message::AuthMethodChanged(AuthMethod::OAuth))
            }
        };

        let tabs = container(
            row![token_tab, oauth_tab].spacing(4)
        )
        .padding(4)
        .width(Length::Fill)
        .style(theme::card_lowest);

        let method_content: Element<'_, Message> = match self.auth_method {
            AuthMethod::Token => self.view_auth_token(),
            AuthMethod::OAuth => self.view_auth_oauth(),
        };

        let card_inner = column![
            title,
            subtitle,
            Space::with_height(20),
            tabs,
            Space::with_height(20),
            method_content,
        ]
        .spacing(8)
        .max_width(460);

        let card = container(
            row![
                accent,
                container(card_inner)
                    .padding(32)
                    .width(Length::Fill)
                    .style(theme::card),
            ]
        )
        .max_width(520);

        // Bottom status bar
        let status_bar = container(
            row![
                text("\u{25CF}")
                    .size(8)
                    .color(if self.is_authenticated { theme::colors::SUCCESS } else { theme::colors::ERROR }),
                text(if self.is_authenticated { "Connected" } else { "Daemon Offline" })
                    .size(10)
                    .font(theme::FONT_MONO)
                    .color(theme::colors::OUTLINE),
                Space::with_width(Length::Fill),
                text("iced-v0.13")
                    .size(10)
                    .font(theme::FONT_MONO)
                    .color(theme::colors::OUTLINE_VARIANT),
            ]
            .spacing(6)
            .align_y(Alignment::Center)
            .padding([0, 24])
        )
        .width(Length::Fill)
        .height(32)
        .center_y(32)
        .style(theme::card_lowest);

        container(
            column![
                Space::with_height(Length::Fill),
                card,
                Space::with_height(Length::Fill),
                status_bar,
            ]
            .align_x(Alignment::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .into()
    }

    fn view_auth_token(&self) -> Element<'_, Message> {
        let token_label = text("ACCESS TOKEN")
            .size(10)
            .font(theme::FONT_MONO)
            .color(theme::colors::OUTLINE);

        let token_input = text_input("ghp_xxxxxxxxxxxx", &self.token_input)
            .on_input(Message::TokenInputChanged)
            .on_submit(Message::LoginSubmit)
            .padding(12)
            .size(13)
            .font(theme::FONT_MONO)
            .secure(true)
            .style(theme::surface_input);

        let mut connect_btn = button(
            row![
                text(if self.is_loading { "Connecting..." } else { "Connect to GitHub" })
                    .size(13)
                    .font(theme::FONT_HEADLINE),
            ]
            .spacing(8)
            .align_y(Alignment::Center)
        )
        .padding([12, 24])
        .width(Length::Fill)
        .style(theme::primary_button);

        if !self.is_loading && !self.token_input.is_empty() {
            connect_btn = connect_btn.on_press(Message::LoginSubmit);
        }

        let error_text: Element<'_, Message> = if let Some(ref status) = self.auth_status {
            text(status.as_str())
                .size(12)
                .color(theme::colors::ERROR)
                .into()
        } else {
            Space::with_height(0).into()
        };

        // Required scopes
        let scopes = container(
            column![
                text("REQUIRED SCOPES")
                    .size(10)
                    .font(theme::FONT_MONO)
                    .color(theme::colors::OUTLINE),
                Space::with_height(8),
                row![
                    scope_item("repo"),
                    scope_item("read:user"),
                    scope_item("read:org"),
                ]
                .spacing(16),
            ]
        )
        .padding(16)
        .width(Length::Fill)
        .style(theme::card_low);

        let create_token_btn = button(
            row![
                text("Create a new token on GitHub").size(12),
                text("\u{2197}").size(12).color(theme::colors::PRIMARY),
            ]
            .spacing(8)
        )
        .padding([10, 0])
        .width(Length::Fill)
        .style(theme::ghost_button)
        .on_press(Message::OpenUrl(
            "https://github.com/settings/tokens/new?scopes=repo,read:user,read:org&description=Gosh%20GitHub%20Backup%20Manager".to_string(),
        ));

        column![
            token_label,
            token_input,
            error_text,
            Space::with_height(8),
            connect_btn,
            Space::with_height(8),
            create_token_btn,
            Space::with_height(12),
            scopes,
        ]
        .spacing(6)
        .into()
    }

    fn view_auth_oauth(&self) -> Element<'_, Message> {
        match &self.oauth_status {
            OAuthStatus::Idle => {
                let description = text(
                    "Sign in through your browser using GitHub Device Flow. No token needed.",
                )
                .size(13)
                .color(theme::colors::ON_SURFACE_VARIANT);

                let sign_in_btn = button(
                    row![
                        text("Sign in with GitHub")
                            .size(13)
                            .font(theme::FONT_HEADLINE),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center)
                )
                .padding([12, 24])
                .width(Length::Fill)
                .style(theme::primary_button)
                .on_press(Message::OAuthStartDeviceFlow);

                column![description, Space::with_height(12), sign_in_btn]
                    .spacing(8)
                    .into()
            }
            OAuthStatus::RequestingCode => {
                column![
                    text("Connecting to GitHub...")
                        .size(14)
                        .color(theme::colors::ON_SURFACE_VARIANT),
                ]
                .spacing(8)
                .into()
            }
            OAuthStatus::WaitingForUser {
                user_code,
                verification_uri,
                expires_at,
                ..
            } => {
                let remaining = expires_at - chrono::Utc::now().timestamp();
                let mins = remaining / 60;
                let secs = remaining % 60;

                // Device code label
                let code_label = text("DEVICE CODE")
                    .size(10)
                    .font(theme::FONT_MONO)
                    .color(theme::colors::OUTLINE);

                // Large code display
                let code_display = container(
                    text(user_code.as_str())
                        .size(28)
                        .font(theme::FONT_MONO)
                        .color(theme::colors::ON_SURFACE)
                )
                .padding([20, 32])
                .width(Length::Fill)
                .center_x(Length::Fill)
                .style(theme::card_lowest);

                // Info box
                let info_box = {
                    let accent = container(Space::new(2, Length::Fill))
                        .style(theme::accent_bar(theme::colors::TERTIARY));

                    let info_content = container(
                        row![
                            text("\u{24D8}")
                                .size(16)
                                .color(theme::colors::TERTIARY),
                            text(format!(
                                "This code will expire in {:02}:{:02}. Make sure you are signed into the correct GitHub account.",
                                mins, secs
                            ))
                            .size(11)
                            .color(theme::colors::ON_SURFACE_VARIANT),
                        ]
                        .spacing(12)
                        .align_y(Alignment::Start)
                    )
                    .padding(16)
                    .width(Length::Fill)
                    .style(theme::card_high);

                    row![accent, info_content]
                };

                let open_btn = button(
                    row![
                        text("\u{2197}").size(16),
                        text("Open GitHub")
                            .size(13)
                            .font(theme::FONT_HEADLINE),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center)
                )
                .padding([12, 24])
                .width(Length::Fill)
                .style(theme::primary_button)
                .on_press(Message::OpenUrl(verification_uri.clone()));

                let cancel_btn = button(
                    text("Cancel")
                        .size(13)
                        .font(theme::FONT_HEADLINE_MEDIUM)
                )
                .padding([10, 24])
                .width(Length::Fill)
                .style(theme::ghost_button)
                .on_press(Message::OAuthCancel);

                column![
                    code_label,
                    Space::with_height(8),
                    code_display,
                    Space::with_height(16),
                    info_box,
                    Space::with_height(16),
                    open_btn,
                    cancel_btn,
                ]
                .spacing(4)
                .align_x(Alignment::Center)
                .into()
            }
            OAuthStatus::Error(msg) => {
                let error = text(msg.as_str())
                    .size(13)
                    .color(theme::colors::ERROR);

                let retry_btn = button(
                    text("Try Again")
                        .size(13)
                        .font(theme::FONT_HEADLINE)
                )
                .padding([12, 24])
                .width(Length::Fill)
                .style(theme::primary_button)
                .on_press(Message::OAuthStartDeviceFlow);

                column![error, Space::with_height(12), retry_btn]
                    .spacing(8)
                    .into()
            }
        }
    }
}

fn scope_item(scope: &str) -> Element<'_, Message> {
    row![
        text("\u{2713}")
            .size(12)
            .color(theme::colors::TERTIARY),
        text(scope)
            .size(11)
            .font(theme::FONT_MONO)
            .color(theme::colors::ON_SURFACE),
    ]
    .spacing(6)
    .align_y(Alignment::Center)
    .into()
}
