use crate::app::{GoshApp, Message};
use crate::types::BackupTab;
use iced::widget::{button, column, container, pick_list, progress_bar, row, scrollable, slider, text, text_input, toggler, Space};
use iced::{Alignment, Element, Length};

impl GoshApp {
    pub fn view_backup(&self) -> Element<'_, Message> {
        let selected_count = self.selected_repos.len();

        // Header
        let header = row![
            column![
                text("Backup Manager").size(24),
                text(format!("{} repositories selected", selected_count)).size(13),
            ]
            .spacing(4),
            Space::with_width(Length::Fill),
            if !self.is_backup_running && selected_count > 0 && !self.backup_options.destination.is_empty() {
                Element::from(
                    button(text("Start Backup").size(13))
                        .padding([10, 20])
                        .style(button::primary)
                        .on_press(Message::BackupStart)
                )
            } else if self.is_backup_running {
                Element::from(
                    button(text("Cancel").size(13))
                        .padding([10, 20])
                        .style(button::danger)
                        .on_press(Message::BackupCancel)
                )
            } else {
                Element::from(Space::with_width(0))
            },
        ]
        .align_y(Alignment::Center);

        // Tabs
        let progress_label = if self.is_backup_running {
            "Progress \u{25CF}"
        } else {
            "Progress"
        };

        let make_tab = |label: String, tab: BackupTab, is_active: bool| -> Element<'static, Message> {
            let btn = button(text(label).size(13))
                .padding([8, 16])
                .on_press(Message::BackupTabChanged(tab));
            if is_active {
                btn.style(button::primary).into()
            } else {
                btn.style(button::secondary).into()
            }
        };

        let active_tab = self.backup_active_tab;
        let tabs = row![
            make_tab("Options".to_string(), BackupTab::Options, active_tab == BackupTab::Options),
            make_tab(progress_label.to_string(), BackupTab::Progress, active_tab == BackupTab::Progress),
            make_tab("History".to_string(), BackupTab::History, active_tab == BackupTab::History),
        ]
        .spacing(4);

        let tab_content: Element<Message> = match self.backup_active_tab {
            BackupTab::Options => self.view_backup_options(),
            BackupTab::Progress => self.view_backup_progress(),
            BackupTab::History => self.view_backup_history(),
        };

        let content = column![
            header,
            Space::with_height(12),
            tabs,
            Space::with_height(12),
            tab_content,
            Space::with_height(20),
        ]
        .spacing(4)
        .padding(24);

        scrollable(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_backup_options(&self) -> Element<'_, Message> {
        // Destination
        let dest_row = row![
            text_input("Select backup destination...", &self.backup_options.destination)
                .padding(8)
                .size(13)
                .width(Length::Fill),
            button(text("Browse").size(13))
                .padding([8, 16])
                .on_press(Message::BackupSelectFolder),
        ]
        .spacing(8);

        let dest_section = container(
            column![
                text("Destination").size(15),
                dest_row,
            ]
            .spacing(8)
        )
        .padding(16)
        .width(Length::Fill)
        .style(container::bordered_box);

        // Clone settings
        let clone_types = vec![
            "full".to_string(),
            "mirror".to_string(),
        ];
        let current_clone = Some(self.backup_options.clone_type.clone());

        let clone_picker = pick_list(
            clone_types,
            current_clone,
            |val| Message::BackupCloneTypeChanged(val),
        )
        .padding(8)
        .text_size(13);

        let concurrent = self.backup_options.max_concurrent.unwrap_or(3) as f64;
        let concurrent_slider = row![
            text(format!("Concurrent: {}", concurrent as u8)).size(13),
            slider(1.0..=5.0, concurrent, |val| {
                Message::BackupConcurrentChanged(val as u8)
            })
            .step(1.0)
            .width(Length::Fill),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let clone_section = container(
            column![
                text("Clone Settings").size(15),
                row![text("Clone type:").size(13), clone_picker].spacing(8).align_y(Alignment::Center),
                concurrent_slider,
            ]
            .spacing(8)
        )
        .padding(16)
        .width(Length::Fill)
        .style(container::bordered_box);

        // Include/Exclude
        let include_forks = self.backup_options.include_forks.unwrap_or(true);
        let include_archived = self.backup_options.include_archived.unwrap_or(false);

        let split_options = vec![
            "none".to_string(),
            "owner".to_string(),
        ];
        let current_split = self.backup_options.split_by.clone().or(Some("owner".to_string()));

        let include_section = container(
            column![
                text("Include / Organize").size(15),
                toggler(include_forks)
                    .label("Include forked repositories")
                    .on_toggle(Message::BackupIncludeForksChanged)
                    .text_size(13)
                    .size(20),
                toggler(include_archived)
                    .label("Include archived repositories")
                    .on_toggle(Message::BackupIncludeArchivedChanged)
                    .text_size(13)
                    .size(20),
                row![
                    text("Organize by:").size(13),
                    pick_list(split_options, current_split, Message::BackupSplitByChanged)
                        .padding(8)
                        .text_size(13),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            ]
            .spacing(8)
        )
        .padding(16)
        .width(Length::Fill)
        .style(container::bordered_box);

        // Archive settings
        let create_zip = self.backup_options.create_zip.unwrap_or(false);
        let compression = self.backup_options.zip_compression.unwrap_or(6) as f64;

        let archive_section = container(
            column![
                text("Archive").size(15),
                toggler(create_zip)
                    .label("Create ZIP archive after backup")
                    .on_toggle(Message::BackupCreateZipChanged)
                    .text_size(13)
                    .size(20),
                if create_zip {
                    Element::from(
                        row![
                            text(format!("Compression: {}", compression as u8)).size(13),
                            slider(0.0..=9.0, compression, |val| {
                                Message::BackupCompressionChanged(val as u8)
                            })
                            .step(1.0)
                            .width(Length::Fill),
                        ]
                        .spacing(12)
                        .align_y(Alignment::Center)
                    )
                } else {
                    Element::from(Space::with_height(0))
                },
            ]
            .spacing(8)
        )
        .padding(16)
        .width(Length::Fill)
        .style(container::bordered_box);

        let settings_row = row![
            column![dest_section, include_section].spacing(12).width(Length::FillPortion(1)),
            column![clone_section, archive_section].spacing(12).width(Length::FillPortion(1)),
        ]
        .spacing(12);

        settings_row.into()
    }

    fn view_backup_progress(&self) -> Element<'_, Message> {
        if let Some(ref progress) = self.backup_progress {
            let total = progress.total_repos as f32;
            let completed = (progress.completed_repos + progress.failed_repos) as f32;
            let pct = if total > 0.0 { completed / total } else { 0.0 };

            let elapsed_ms = chrono::Utc::now().timestamp_millis() - progress.start_time;
            let elapsed_secs = elapsed_ms / 1000;
            let elapsed_str = format!("{}m {}s", elapsed_secs / 60, elapsed_secs % 60);

            let remaining_str = if pct > 0.0 && pct < 1.0 {
                let total_est = elapsed_ms as f32 / pct;
                let remaining = (total_est - elapsed_ms as f32) / 1000.0;
                format!("~{}m {}s", remaining as i64 / 60, remaining as i64 % 60)
            } else {
                "---".to_string()
            };

            let overall = column![
                text(format!("Overall Progress: {:.0}%", pct * 100.0)).size(15),
                progress_bar(0.0..=1.0, pct).height(8),
                row![
                    text(format!("Completed: {}", progress.completed_repos)).size(12),
                    text(format!("Failed: {}", progress.failed_repos)).size(12),
                    text(format!("Remaining: {}", progress.total_repos - progress.completed_repos - progress.failed_repos)).size(12),
                    Space::with_width(Length::Fill),
                    text(format!("Elapsed: {}", elapsed_str)).size(12),
                    text(format!("Remaining: {}", remaining_str)).size(12),
                ]
                .spacing(16),
            ]
            .spacing(8);

            let current = if let Some(ref repo) = progress.current_repo {
                text(format!("Currently: {}", repo)).size(13)
            } else if !progress.is_running {
                text("Backup complete").size(13)
            } else {
                text("Starting...").size(13)
            };

            // Per-repo status list
            let mut repo_list = column![].spacing(4);
            for rp in &progress.repos {
                let icon = match rp.status.as_str() {
                    "complete" => "\u{2713}",
                    "failed" => "\u{2717}",
                    "cloning" => "\u{21BB}",
                    _ => "\u{25CB}",
                };

                let status_color = match rp.status.as_str() {
                    "complete" => iced::Color::from_rgb(0.2, 0.8, 0.2),
                    "failed" => iced::Color::from_rgb(0.9, 0.3, 0.3),
                    "cloning" => iced::Color::from_rgb(0.3, 0.6, 0.9),
                    _ => iced::Color::from_rgb(0.5, 0.5, 0.5),
                };

                let error_text = if let Some(ref err) = rp.error {
                    text(err.as_str()).size(11).color(iced::Color::from_rgb(0.9, 0.3, 0.3))
                } else {
                    text("").size(11)
                };

                let repo_row = container(
                    column![
                        row![
                            text(icon).size(14).color(status_color),
                            text(&rp.repo_name).size(12),
                            Space::with_width(Length::Fill),
                            text(&rp.status).size(11),
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center),
                        error_text,
                    ]
                    .spacing(2)
                )
                .padding([6, 12])
                .width(Length::Fill)
                .style(container::bordered_box);

                repo_list = repo_list.push(repo_row);
            }

            column![
                overall,
                current,
                Space::with_height(8),
                repo_list,
            ]
            .spacing(8)
            .into()
        } else {
            container(
                column![
                    text("No backup in progress").size(16),
                    text("Configure options and start a backup to see progress here.").size(13),
                ]
                .spacing(8)
            )
            .padding(40)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .into()
        }
    }

    fn view_backup_history(&self) -> Element<'_, Message> {
        if self.backup_history.is_empty() {
            return container(
                column![
                    text("No backup history").size(16),
                    text("Completed backups will appear here.").size(13),
                ]
                .spacing(8)
            )
            .padding(40)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .into();
        }

        let mut list = column![].spacing(8);
        for entry in &self.backup_history {
            let status_icon = match entry.status.as_str() {
                "complete" => "\u{2713}",
                "partial" => "\u{26A0}",
                "failed" => "\u{2717}",
                _ => "\u{25CB}",
            };

            let duration_secs = entry.duration / 1000;
            let duration_str = format!("{}m {}s", duration_secs / 60, duration_secs % 60);

            let date_display = entry.date.split('T').next().unwrap_or(&entry.date);

            let entry_row = container(
                row![
                    column![
                        row![
                            text(status_icon).size(14),
                            text(date_display).size(13),
                            text(format!("| {} repos", entry.repo_count)).size(12),
                            text(format!("| {}", duration_str)).size(12),
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center),
                        text(&entry.destination).size(11),
                    ]
                    .spacing(4),
                    Space::with_width(Length::Fill),
                    button(text("Open").size(12))
                        .padding([6, 12])
                        .style(button::secondary)
                        .on_press(Message::BackupOpenFolder(entry.destination.clone())),
                ]
                .align_y(Alignment::Center)
            )
            .padding(12)
            .width(Length::Fill)
            .style(container::bordered_box);

            list = list.push(entry_row);
        }

        list.into()
    }
}
