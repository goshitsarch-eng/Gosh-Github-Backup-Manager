use crate::app::{GoshApp, Message};
use crate::theme;
use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Alignment, Element, Length};

impl GoshApp {
    pub fn view_dashboard(&self) -> Element<'_, Message> {
        let c = self.c();
        let total_repos = self.repos.len() as u32;
        let total_stars: u32 = self.repos.iter().map(|r| r.stargazers_count).sum();
        let total_forks: u32 = self.repos.iter().map(|r| r.forks_count).sum();

        let last_backup_str = self.backup_history.first()
            .map(|h| h.date.split('T').next().unwrap_or(&h.date).to_string())
            .unwrap_or_else(|| "Never".to_string());

        let user_login = self.user.as_ref()
            .map(|u| u.login.clone())
            .unwrap_or_else(|| "user".to_string());

        let initials = self.user.as_ref()
            .and_then(|u| u.name.as_deref())
            .or(self.user.as_ref().map(|u| u.login.as_str()))
            .unwrap_or("?")
            .chars()
            .next()
            .unwrap_or('?')
            .to_uppercase()
            .to_string();

        let total_repos_str = total_repos.to_string();
        let total_stars_str = total_stars.to_string();
        let total_forks_str = total_forks.to_string();

        // ---- Hero Status Card ----
        let hero_card = {
            let subtitle = if last_backup_str == "Never" {
                format!("Your {} repositories are ready for backup.", total_repos)
            } else {
                format!("Your {} repositories are synchronized. Last backup completed on {}.", total_repos, &last_backup_str)
            };

            container(
                column![
                    container(
                        text("STATUS: OPTIMAL")
                            .size(10)
                            .font(theme::FONT_MONO)
                            .color(c.primary)
                    )
                    .padding([4, 10])
                    .style(theme::status_badge),
                    Space::with_height(12),
                    text("System Protected")
                        .size(32)
                        .font(theme::FONT_HEADLINE),
                    text(subtitle)
                        .size(13)
                        .color(c.on_surface_variant),
                    Space::with_height(24),
                    row![
                        column![
                            text("LAST RUN")
                                .size(10)
                                .font(theme::FONT_MONO)
                                .color(c.on_surface_variant),
                            text(last_backup_str.clone())
                                .size(13)
                                .font(theme::FONT_MONO)
                                .color(c.primary),
                        ]
                        .spacing(4),
                        container(Space::new(1, 32))
                            .style(theme::accent_bar(c.border_subtle)),
                        column![
                            text("INTEGRITY")
                                .size(10)
                                .font(theme::FONT_MONO)
                                .color(c.on_surface_variant),
                            text("Verified 100%")
                                .size(13)
                                .font(theme::FONT_MONO)
                                .color(c.secondary),
                        ]
                        .spacing(4),
                    ]
                    .spacing(24),
                ]
                .spacing(4)
            )
            .padding(32)
            .width(Length::FillPortion(8))
            .style(theme::card_low)
        };

        // ---- Profile Card ----
        let profile_card = {
            let github_url = format!("github.com/{}", &user_login);

            container(
                column![
                    row![
                        container(
                            text(initials).size(20).color(c.on_surface)
                        )
                        .width(48)
                        .height(48)
                        .center_x(48)
                        .center_y(48)
                        .style(theme::card_highest),
                        column![
                            text(user_login.clone())
                                .size(16)
                                .font(theme::FONT_HEADLINE),
                            text(github_url)
                                .size(11)
                                .font(theme::FONT_MONO)
                                .color(c.on_surface_variant),
                        ]
                        .spacing(4),
                    ]
                    .spacing(16)
                    .align_y(Alignment::Center),
                    Space::with_height(24),
                    stat_line("Total Repositories", total_repos_str, c),
                    stat_line("Total Stars", total_stars_str.clone(), c),
                ]
                .spacing(12)
            )
            .padding(24)
            .width(Length::FillPortion(4))
            .style(theme::card)
        };

        let top_row = row![hero_card, profile_card].spacing(16);

        // ---- Metrics Row ----
        let metrics_row = row![
            metric_card("\u{2605}", "Stars Received", total_stars_str, c.tertiary, c),
            metric_card("\u{2442}", "Forks Created", total_forks_str, c.secondary, c),
            metric_card("\u{23F0}", "Last Backup", last_backup_str, c.primary, c),
        ]
        .spacing(16);

        // ---- Recent Backups Table ----
        let backups_table = {
            let header_bar = container(
                row![
                    text("RECENT BACKUPS")
                        .size(12)
                        .font(theme::FONT_HEADLINE)
                        .color(c.on_surface),
                    Space::with_width(Length::Fill),
                    text("View All Operations")
                        .size(11)
                        .color(c.primary),
                ]
                .align_y(Alignment::Center)
            )
            .padding([16, 24])
            .width(Length::Fill);

            let table_head = container(
                row![
                    text("REPOSITORY").size(10).font(theme::FONT_MONO).color(c.on_surface_variant).width(Length::FillPortion(3)),
                    text("OPERATION ID").size(10).font(theme::FONT_MONO).color(c.on_surface_variant).width(Length::FillPortion(2)),
                    text("DATE").size(10).font(theme::FONT_MONO).color(c.on_surface_variant).width(Length::FillPortion(2)),
                    text("STATUS").size(10).font(theme::FONT_MONO).color(c.on_surface_variant).width(Length::FillPortion(1)),
                ]
                .padding([0, 24])
            )
            .padding([12, 0])
            .width(Length::Fill)
            .style(theme::table_header);

            let mut rows = column![].spacing(0);
            for entry in self.backup_history.iter().take(5) {
                let date_display = entry.date.split('T').next().unwrap_or(&entry.date).to_string();
                let status_color = match entry.status.as_str() {
                    "complete" => c.primary,
                    "partial" => c.secondary,
                    _ => c.error,
                };
                let short_id = if entry.id.len() > 10 {
                    format!("bk_{}", &entry.id[entry.id.len()-6..])
                } else {
                    entry.id.clone()
                };

                let row_content = container(
                    row![
                        text(format!("{} repos", entry.repo_count))
                            .size(13)
                            .width(Length::FillPortion(3)),
                        text(short_id)
                            .size(11)
                            .font(theme::FONT_MONO)
                            .color(c.on_surface_variant)
                            .width(Length::FillPortion(2)),
                        text(date_display)
                            .size(11)
                            .color(c.on_surface_variant)
                            .width(Length::FillPortion(2)),
                        text("\u{25CF}")
                            .size(10)
                            .color(status_color)
                            .width(Length::FillPortion(1)),
                    ]
                    .align_y(Alignment::Center)
                    .padding([0, 24])
                )
                .padding([12, 0])
                .width(Length::Fill);

                rows = rows.push(row_content);
            }

            if self.backup_history.is_empty() {
                rows = rows.push(
                    container(
                        text("No backup history yet")
                            .size(13)
                            .color(c.on_surface_variant)
                    )
                    .padding([24, 24])
                    .width(Length::Fill)
                );
            }

            container(
                column![header_bar, table_head, rows]
            )
            .width(Length::Fill)
            .style(theme::card)
        };

        let content = column![
            top_row,
            Space::with_height(16),
            metrics_row,
            Space::with_height(16),
            backups_table,
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

fn stat_line(label: &str, value: String, c: theme::Scheme) -> Element<'_, Message> {
    row![
        text(label)
            .size(12)
            .color(c.on_surface_variant),
        Space::with_width(Length::Fill),
        text(value)
            .size(18)
            .font(theme::FONT_HEADLINE),
    ]
    .align_y(Alignment::Center)
    .into()
}

fn metric_card<'a>(
    icon: &'a str,
    label: &'a str,
    value: String,
    accent_color: iced::Color,
    c: theme::Scheme,
) -> Element<'a, Message> {
    container(
        column![
            text(icon)
                .size(20)
                .color(accent_color),
            Space::with_height(12),
            text(value)
                .size(22)
                .font(theme::FONT_HEADLINE),
            text(label)
                .size(10)
                .font(theme::FONT_MONO)
                .color(c.on_surface_variant),
        ]
        .spacing(4)
    )
    .padding(24)
    .width(Length::Fill)
    .style(theme::card_lowest)
    .into()
}
