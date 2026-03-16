use crate::app::{GoshApp, Message};
use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Element, Length};

impl GoshApp {
    pub fn view_about(&self) -> Element<'_, Message> {
        let header = text("About").size(24);

        let app_info = container(
            column![
                text("Gosh GitHub Backup Manager").size(20),
                text("Version 2.0.0").size(14),
                Space::with_height(8),
                text("A cross-platform desktop application for backing up your GitHub repositories to local storage.").size(13),
                Space::with_height(8),
                text("Built with pure Rust").size(13),
                text("License: AGPL-3.0").size(13),
            ]
            .spacing(4)
        )
        .padding(20)
        .width(Length::Fill)
        .style(container::bordered_box);

        let tech_section = container(
            column![
                text("Technology").size(16),
                Space::with_height(8),
                tech_item("GUI Framework", "iced 0.13"),
                tech_item("Language", "Rust"),
                tech_item("HTTP Client", "reqwest"),
                tech_item("Git Operations", "git2 (libgit2)"),
                tech_item("Archive", "zip"),
                tech_item("Async Runtime", "tokio"),
            ]
            .spacing(4)
        )
        .padding(20)
        .width(Length::Fill)
        .style(container::bordered_box);

        let links = row![
            button(text("View on GitHub").size(13))
                .padding([8, 16])
                .style(button::primary)
                .on_press(Message::OpenUrl("https://github.com/goshitsarch-eng/Gosh-Github-Backup-Manager".to_string())),
            button(text("Report Issue").size(13))
                .padding([8, 16])
                .style(button::secondary)
                .on_press(Message::OpenUrl("https://github.com/goshitsarch-eng/Gosh-Github-Backup-Manager/issues".to_string())),
        ]
        .spacing(8);

        let content = column![
            header,
            Space::with_height(16),
            app_info,
            Space::with_height(12),
            tech_section,
            Space::with_height(16),
            links,
            Space::with_height(20),
        ]
        .padding(24);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn tech_item<'a>(label: &'a str, value: &'a str) -> Element<'a, Message> {
    row![
        text(label).size(13).width(Length::FillPortion(1)),
        text(value).size(13).width(Length::FillPortion(2)),
    ]
    .align_y(Alignment::Center)
    .into()
}
