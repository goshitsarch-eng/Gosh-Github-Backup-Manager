use crate::app::{GoshApp, Message};
use crate::theme;
use crate::types::AppTheme;
use iced::widget::{button, column, container, row, scrollable, text, toggler, Space};
use iced::{Alignment, Element, Length};

impl GoshApp {
    pub fn view_settings(&self) -> Element<'_, Message> {
        let c = self.c();

        let header = column![
            text("Settings")
                .size(28)
                .font(theme::FONT_HEADLINE),
            text("Manage your app preferences and account.")
                .size(13)
                .color(c.on_surface_variant),
        ]
        .spacing(8);

        // Theme section
        let make_theme_btn = |label: String, app_theme: AppTheme, is_active: bool| -> Element<'static, Message> {
            let btn = button(
                text(label)
                    .size(12)
                    .font(theme::FONT_HEADLINE)
            )
            .padding([8, 20])
            .on_press(Message::ThemeChanged(app_theme));
            if is_active {
                btn.style(theme::tab_active).into()
            } else {
                btn.style(theme::tab_inactive).into()
            }
        };

        let ct = self.current_theme;
        let theme_section = container(
            column![
                section_header("Appearance"),
                row![
                    make_theme_btn("Light".to_string(), AppTheme::Light, ct == AppTheme::Light),
                    make_theme_btn("Dark".to_string(), AppTheme::Dark, ct == AppTheme::Dark),
                    make_theme_btn("System".to_string(), AppTheme::System, ct == AppTheme::System),
                ]
                .spacing(8),
            ]
            .spacing(12)
        )
        .padding(20)
        .width(Length::Fill)
        .style(theme::card);

        // Notifications
        let notif_section = container(
            column![
                section_header("Notifications"),
                toggler(self.settings.notifications)
                    .label("Enable notifications")
                    .on_toggle(Message::NotificationsChanged)
                    .text_size(13)
                    .size(20),
            ]
            .spacing(12)
        )
        .padding(20)
        .width(Length::Fill)
        .style(theme::card);

        // Backup defaults
        let default_location = &self.settings.default_backup_location;
        let backup_defaults = container(
            column![
                section_header("Backup Defaults"),
                row![
                    text(if default_location.is_empty() {
                        "No default location set"
                    } else {
                        default_location.as_str()
                    })
                    .size(12)
                    .font(theme::FONT_MONO)
                    .color(c.on_surface_variant),
                    Space::with_width(Length::Fill),
                    button(
                        text("Browse")
                            .size(11)
                            .font(theme::FONT_MONO)
                    )
                    .padding([6, 16])
                    .style(theme::ghost_button)
                    .on_press(Message::DefaultFolderSelect),
                ]
                .align_y(Alignment::Center),
            ]
            .spacing(12)
        )
        .padding(20)
        .width(Length::Fill)
        .style(theme::card);

        // Account section
        let account_section = container(
            column![
                section_header("Account"),
                if let Some(ref user) = self.user {
                    Element::from(
                        row![
                            column![
                                text(format!("Signed in as @{}", user.login))
                                    .size(13),
                                text("Connected via Personal Access Token or OAuth")
                                    .size(11)
                                    .color(c.on_surface_variant),
                            ]
                            .spacing(4),
                            Space::with_width(Length::Fill),
                            button(
                                text("Disconnect")
                                    .size(11)
                                    .font(theme::FONT_HEADLINE)
                            )
                            .padding([8, 16])
                            .style(theme::danger_button)
                            .on_press(Message::LogoutRequested),
                        ]
                        .align_y(Alignment::Center)
                    )
                } else {
                    Element::from(
                        text("Not signed in")
                            .size(13)
                            .color(c.on_surface_variant)
                    )
                },
            ]
            .spacing(12)
        )
        .padding(20)
        .width(Length::Fill)
        .style(theme::card);

        // Data management
        let data_section = container(
            column![
                section_header("Data Management"),
                button(
                    text("Clear Backup History")
                        .size(12)
                        .font(theme::FONT_MONO)
                )
                .padding([8, 16])
                .style(theme::ghost_button)
                .on_press(Message::ClearBackupHistory),
            ]
            .spacing(12)
        )
        .padding(20)
        .width(Length::Fill)
        .style(theme::card);

        // Logout confirmation overlay
        let logout_overlay: Element<Message> = if self.logout_confirm_visible {
            container(
                container(
                    column![
                        text("Disconnect Account")
                            .size(18)
                            .font(theme::FONT_HEADLINE),
                        text("Are you sure you want to disconnect? Your token will be removed.")
                            .size(13)
                            .color(c.on_surface_variant),
                        Space::with_height(16),
                        row![
                            button(
                                text("Cancel")
                                    .size(12)
                                    .font(theme::FONT_HEADLINE)
                            )
                            .padding([10, 20])
                            .style(theme::ghost_button)
                            .on_press(Message::LogoutCancelled),
                            button(
                                text("Disconnect")
                                    .size(12)
                                    .font(theme::FONT_HEADLINE)
                            )
                            .padding([10, 20])
                            .style(theme::danger_button)
                            .on_press(Message::LogoutConfirmed),
                        ]
                        .spacing(8),
                    ]
                    .spacing(8)
                )
                .padding(28)
                .max_width(420)
                .style(theme::card)
            )
            .width(Length::Fill)
            .center_x(Length::Fill)
            .into()
        } else {
            Space::with_height(0).into()
        };

        let content = column![
            header,
            Space::with_height(24),
            logout_overlay,
            row![
                column![theme_section, notif_section, data_section]
                    .spacing(16)
                    .width(Length::FillPortion(1)),
                column![backup_defaults, account_section]
                    .spacing(16)
                    .width(Length::FillPortion(1)),
            ]
            .spacing(16),
            Space::with_height(24),
        ]
        .spacing(0)
        .padding(32);

        scrollable(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn section_header(label: &str) -> Element<'_, Message> {
    text(label)
        .size(14)
        .font(theme::FONT_HEADLINE)
        .into()
}
