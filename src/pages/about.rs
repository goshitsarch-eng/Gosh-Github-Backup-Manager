use crate::app::{GoshApp, Message};
use crate::theme;
use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Element, Length};

impl GoshApp {
    pub fn view_about(&self) -> Element<'_, Message> {
        let c = self.c();

        let app_info = container(
            column![
                text("Gosh GitHub Backup Manager")
                    .size(20)
                    .font(theme::FONT_HEADLINE),
                text("Version 2.1.0")
                    .size(13)
                    .font(theme::FONT_MONO)
                    .color(c.primary),
                Space::with_height(12),
                text("A cross-platform desktop application for backing up your GitHub repositories to local storage.")
                    .size(13)
                    .color(c.on_surface_variant),
                Space::with_height(8),
                text("Built with pure Rust")
                    .size(12)
                    .font(theme::FONT_MONO)
                    .color(c.outline),
                text("License: AGPL-3.0")
                    .size(12)
                    .font(theme::FONT_MONO)
                    .color(c.outline),
            ]
            .spacing(4)
        )
        .padding(24)
        .width(Length::Fill)
        .style(theme::card);

        let tech_section = container(
            column![
                text("Technology")
                    .size(14)
                    .font(theme::FONT_HEADLINE),
                Space::with_height(12),
                tech_item("GUI Framework", "iced 0.13", c),
                tech_item("Language", "Rust", c),
                tech_item("HTTP Client", "reqwest", c),
                tech_item("Git Operations", "git2 (libgit2)", c),
                tech_item("Archive", "zip", c),
                tech_item("Async Runtime", "tokio", c),
            ]
            .spacing(6)
        )
        .padding(24)
        .width(Length::Fill)
        .style(theme::card);

        let links = row![
            button(
                text("View on GitHub")
                    .size(12)
                    .font(theme::FONT_HEADLINE)
            )
            .padding([10, 20])
            .style(theme::primary_button)
            .on_press(Message::OpenUrl("https://github.com/goshitsarch-eng/Gosh-Github-Backup-Manager".to_string())),
            button(
                text("Report Issue")
                    .size(12)
                    .font(theme::FONT_HEADLINE)
            )
            .padding([10, 20])
            .style(theme::ghost_button)
            .on_press(Message::OpenUrl("https://github.com/goshitsarch-eng/Gosh-Github-Backup-Manager/issues".to_string())),
        ]
        .spacing(12);

        let content = column![
            text("About")
                .size(28)
                .font(theme::FONT_HEADLINE),
            Space::with_height(24),
            app_info,
            Space::with_height(16),
            tech_section,
            Space::with_height(20),
            links,
            Space::with_height(24),
        ]
        .padding(32);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn tech_item<'a>(label: &'a str, value: &'a str, c: theme::Scheme) -> Element<'a, Message> {
    row![
        text(label)
            .size(12)
            .color(c.on_surface_variant)
            .width(Length::FillPortion(1)),
        text(value)
            .size(12)
            .font(theme::FONT_MONO)
            .color(c.primary)
            .width(Length::FillPortion(2)),
    ]
    .align_y(Alignment::Center)
    .into()
}
