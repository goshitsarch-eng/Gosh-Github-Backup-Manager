use crate::app::{GoshApp, Message};
use crate::types::AppTheme;
use iced::widget::{button, column, container, row, scrollable, text, toggler, Space};
use iced::{Alignment, Element, Length};

impl GoshApp {
    pub fn view_settings(&self) -> Element<'_, Message> {
        let header = text("Settings").size(24);
        let subtitle = text("Manage your app preferences").size(13);

        // Theme section
        let make_theme_btn = |label: String, theme: AppTheme, is_active: bool| -> Element<'static, Message> {
            let btn = button(text(label).size(13))
                .padding([8, 16])
                .on_press(Message::ThemeChanged(theme));
            if is_active {
                btn.style(button::primary).into()
            } else {
                btn.style(button::secondary).into()
            }
        };

        let ct = self.current_theme;
        let theme_section = container(
            column![
                text("Appearance").size(15),
                row![
                    make_theme_btn("Light".to_string(), AppTheme::Light, ct == AppTheme::Light),
                    make_theme_btn("Dark".to_string(), AppTheme::Dark, ct == AppTheme::Dark),
                    make_theme_btn("System".to_string(), AppTheme::System, ct == AppTheme::System),
                ]
                .spacing(8),
            ]
            .spacing(8)
        )
        .padding(16)
        .width(Length::Fill)
        .style(container::bordered_box);

        // Backup defaults section
        let default_location = &self.settings.default_backup_location;
        let backup_defaults = container(
            column![
                text("Backup Defaults").size(15),
                row![
                    text(if default_location.is_empty() {
                        "No default location set"
                    } else {
                        default_location.as_str()
                    })
                    .size(12),
                    Space::with_width(Length::Fill),
                    button(text("Browse").size(12))
                        .padding([6, 12])
                        .on_press(Message::DefaultFolderSelect),
                ]
                .align_y(Alignment::Center),
            ]
            .spacing(8)
        )
        .padding(16)
        .width(Length::Fill)
        .style(container::bordered_box);

        // Notifications
        let notif_section = container(
            column![
                text("Notifications").size(15),
                toggler(self.settings.notifications)
                    .label("Enable notifications")
                    .on_toggle(Message::NotificationsChanged)
                    .text_size(13)
                    .size(20),
            ]
            .spacing(8)
        )
        .padding(16)
        .width(Length::Fill)
        .style(container::bordered_box);

        // Account section
        let account_section = container(
            column![
                text("Account").size(15),
                if let Some(ref user) = self.user {
                    Element::from(
                        row![
                            text(format!("Signed in as @{}", user.login)).size(13),
                            Space::with_width(Length::Fill),
                            button(text("Disconnect").size(12))
                                .padding([6, 12])
                                .style(button::danger)
                                .on_press(Message::LogoutRequested),
                        ]
                        .align_y(Alignment::Center)
                    )
                } else {
                    Element::from(text("Not signed in").size(13))
                },
            ]
            .spacing(8)
        )
        .padding(16)
        .width(Length::Fill)
        .style(container::bordered_box);

        // Data management
        let data_section = container(
            column![
                text("Data Management").size(15),
                row![
                    button(text("Clear Backup History").size(12))
                        .padding([6, 12])
                        .style(button::secondary)
                        .on_press(Message::ClearBackupHistory),
                ],
            ]
            .spacing(8)
        )
        .padding(16)
        .width(Length::Fill)
        .style(container::bordered_box);

        // Logout confirmation overlay
        let logout_overlay: Element<Message> = if self.logout_confirm_visible {
            container(
                container(
                    column![
                        text("Disconnect Account").size(18),
                        text("Are you sure you want to disconnect? Your token will be removed.").size(13),
                        Space::with_height(12),
                        row![
                            button(text("Cancel").size(13))
                                .padding([8, 16])
                                .style(button::secondary)
                                .on_press(Message::LogoutCancelled),
                            button(text("Disconnect").size(13))
                                .padding([8, 16])
                                .style(button::danger)
                                .on_press(Message::LogoutConfirmed),
                        ]
                        .spacing(8),
                    ]
                    .spacing(8)
                )
                .padding(24)
                .max_width(400)
                .style(container::bordered_box)
            )
            .width(Length::Fill)
            .center_x(Length::Fill)
            .into()
        } else {
            Space::with_height(0).into()
        };

        let content = column![
            header,
            subtitle,
            Space::with_height(16),
            logout_overlay,
            row![
                column![theme_section, notif_section, data_section].spacing(12).width(Length::FillPortion(1)),
                column![backup_defaults, account_section].spacing(12).width(Length::FillPortion(1)),
            ]
            .spacing(12),
            Space::with_height(20),
        ]
        .spacing(4)
        .padding(24);

        scrollable(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
