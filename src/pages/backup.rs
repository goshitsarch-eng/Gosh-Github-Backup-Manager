use crate::app::{GoshApp, Message};
use crate::theme;
use crate::types::BackupTab;
use iced::widget::{button, column, container, progress_bar, row, scrollable, slider, text, text_input, toggler, Space};
use iced::{Alignment, Element, Length};

impl GoshApp {
    pub fn view_backup(&self) -> Element<'_, Message> {
        let c = self.c();
        let selected_count = self.selected_repos.len();

        // Header
        let header = column![
            text("Backup Configurations")
                .size(28)
                .font(theme::FONT_HEADLINE),
            text("Configure the synchronization parameters for your local repository mirrors.")
                .size(13)
                .color(c.on_surface_variant),
        ]
        .spacing(8);

        // Tabs
        let progress_label = if self.is_backup_running {
            "Progress \u{25CF}"
        } else {
            "Progress"
        };

        let make_tab = |label: String, tab: BackupTab, is_active: bool| -> Element<'static, Message> {
            let btn = button(
                text(label)
                    .size(11)
                    .font(theme::FONT_HEADLINE)
            )
            .padding([8, 20]);
            if is_active {
                btn.style(theme::tab_active)
                    .on_press(Message::BackupTabChanged(tab))
                    .into()
            } else {
                btn.style(theme::tab_inactive)
                    .on_press(Message::BackupTabChanged(tab))
                    .into()
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

        // Footer action bar
        let footer: Element<Message> = if self.backup_active_tab == BackupTab::Options {
            let status_indicator = row![
                text("\u{25CF}")
                    .size(8)
                    .color(if self.is_backup_running { c.primary } else { c.secondary }),
                text(if self.is_backup_running { "Backup in progress" } else { "System Idle - Ready for process" })
                    .size(11)
                    .font(theme::FONT_MONO)
                    .color(c.on_surface_variant),
            ]
            .spacing(8)
            .align_y(Alignment::Center);

            let action_btn = if self.is_backup_running {
                Element::from(
                    button(
                        row![
                            text("\u{25A0}").size(14),
                            text("Stop Operation")
                                .size(13)
                                .font(theme::FONT_HEADLINE),
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center)
                    )
                    .padding([14, 32])
                    .style(theme::danger_button)
                    .on_press(Message::BackupCancel)
                )
            } else {
                Element::from(
                    button(
                        row![
                            text("Start Backup")
                                .size(13)
                                .font(theme::FONT_HEADLINE),
                            text("\u{2192}").size(16),
                        ]
                        .spacing(12)
                        .align_y(Alignment::Center)
                    )
                    .padding([14, 32])
                    .style(theme::primary_button)
                    .on_press(Message::BackupStart)
                )
            };

            container(
                row![
                    status_indicator,
                    Space::with_width(Length::Fill),
                    action_btn,
                ]
                .align_y(Alignment::Center)
            )
            .padding([24, 0])
            .width(Length::Fill)
            .style(move |_: &iced::Theme| iced::widget::container::Style {
                border: iced::Border {
                    color: c.border_subtle,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
        } else {
            Space::with_height(0).into()
        };

        let content = column![
            header,
            Space::with_height(20),
            tabs,
            Space::with_height(16),
            tab_content,
            footer,
            Space::with_height(24),
        ]
        .spacing(0)
        .padding(32);

        scrollable(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_backup_options(&self) -> Element<'_, Message> {
        let c = self.c();

        // ---- Left Column (7/12) ----

        // Backup Mode Selection
        let mode_section = {
            let is_full = self.backup_options.clone_type == "full";

            let full_btn = button(
                column![
                    text("Full Clone")
                        .size(13)
                        .font(theme::FONT_HEADLINE)
                        .color(if is_full { c.on_surface } else { c.on_surface_variant }),
                    text("Downloads all branches and entire commit history.")
                        .size(11)
                        .color(c.on_surface_variant),
                ]
                .spacing(6)
            )
            .padding(16)
            .width(Length::Fill)
            .style(if is_full { theme::mode_card_active } else { theme::mode_card_inactive })
            .on_press(Message::BackupCloneTypeChanged("full".to_string()));

            let mirror_btn = button(
                column![
                    text("Mirror Clone")
                        .size(13)
                        .font(theme::FONT_HEADLINE)
                        .color(if !is_full { c.on_surface } else { c.on_surface_variant }),
                    text("Exact remote mapping with refs/notes and server-side structure.")
                        .size(11)
                        .color(c.on_surface_variant),
                ]
                .spacing(6)
            )
            .padding(16)
            .width(Length::Fill)
            .style(if !is_full { theme::mode_card_active } else { theme::mode_card_inactive })
            .on_press(Message::BackupCloneTypeChanged("mirror".to_string()));

            let full_card: Element<'_, Message> = full_btn.into();

            let mirror_card: Element<'_, Message> = mirror_btn.into();

            column![
                section_label("BACKUP MODE", c),
                row![full_card, mirror_card].spacing(12),
            ]
            .spacing(12)
        };

        // Target Folder
        let dest_section = column![
            section_label("TARGET FOLDER", c),
            text_input("Path to backup directory...", &self.backup_options.destination)
                .padding(12)
                .size(13)
                .font(theme::FONT_MONO)
                .style(theme::surface_input)
                .width(Length::Fill),
            button(
                text("Browse")
                    .size(11)
                    .font(theme::FONT_MONO)
            )
            .padding([8, 16])
            .style(theme::ghost_button)
            .on_press(Message::BackupSelectFolder),
        ]
        .spacing(8);

        // Performance
        let concurrent = self.backup_options.max_concurrent.unwrap_or(3) as f64;
        let perf_section = column![
            row![
                section_label("PERFORMANCE", c),
                Space::with_width(Length::Fill),
                text("CPU THREAD OPTIMIZATION")
                    .size(10)
                    .font(theme::FONT_MONO)
                    .color(c.tertiary),
            ]
            .align_y(Alignment::End),
            container(
                column![
                    row![
                        text("Concurrent Operations").size(13),
                        Space::with_width(Length::Fill),
                        text(format!("{}", concurrent as u8))
                            .size(13)
                            .font(theme::FONT_MONO)
                            .color(c.primary),
                    ],
                    slider(1.0..=8.0, concurrent, |val| {
                        Message::BackupConcurrentChanged(val as u8)
                    })
                    .step(1.0)
                    .width(Length::Fill),
                    row![
                        text("LOW IMPACT (1)")
                            .size(10)
                            .font(theme::FONT_MONO)
                            .color(c.outline),
                        Space::with_width(Length::Fill),
                        text("AGGRESSIVE (8)")
                            .size(10)
                            .font(theme::FONT_MONO)
                            .color(c.outline),
                    ],
                ]
                .spacing(8)
            )
            .padding(20)
            .style(theme::card),
        ]
        .spacing(12);

        // Include/Exclude togglers
        let include_forks = self.backup_options.include_forks.unwrap_or(true);
        let include_archived = self.backup_options.include_archived.unwrap_or(false);

        let include_section = column![
            section_label("INCLUDE", c),
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
        ]
        .spacing(12);

        let left_col = column![
            mode_section,
            Space::with_height(24),
            dest_section,
            Space::with_height(24),
            perf_section,
            Space::with_height(24),
            include_section,
        ]
        .width(Length::FillPortion(7));

        // ---- Right Column (5/12) ----

        // Archive Options
        let create_zip = self.backup_options.create_zip.unwrap_or(false);
        let compression = self.backup_options.zip_compression.unwrap_or(6) as f64;

        let archive_section = container(
            column![
                section_label("ARCHIVE OPTIONS", c),
                Space::with_height(8),
                row![
                    column![
                        text("Create Zip Archives")
                            .size(13)
                            .font(theme::FONT_HEADLINE),
                        text("Compress repository after cloning")
                            .size(11)
                            .color(c.on_surface_variant),
                    ]
                    .spacing(4)
                    .width(Length::Fill),
                    toggler(create_zip)
                        .on_toggle(Message::BackupCreateZipChanged)
                        .size(20),
                ]
                .align_y(Alignment::Center),
                if create_zip {
                    Element::from(
                        column![
                            Space::with_height(16),
                            row![
                                text("Compression Level").size(12),
                                Space::with_width(Length::Fill),
                                text(format!("Level {}", compression as u8))
                                    .size(11)
                                    .font(theme::FONT_MONO)
                                    .color(c.tertiary),
                            ],
                            slider(0.0..=9.0, compression, |val| {
                                Message::BackupCompressionChanged(val as u8)
                            })
                            .step(1.0)
                            .width(Length::Fill),
                        ]
                        .spacing(8)
                    )
                } else {
                    Element::from(Space::with_height(0))
                },
            ]
            .spacing(8)
        )
        .padding(24)
        .width(Length::Fill)
        .style(theme::card);

        // Info card
        let info_card = container(
            column![
                row![
                    text("\u{24D8}")
                        .size(16)
                        .color(c.tertiary),
                    text("Optimization Note")
                        .size(13)
                        .font(theme::FONT_HEADLINE_MEDIUM),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
                Space::with_height(8),
                text("Higher compression levels significantly increase CPU usage during the archival phase. We recommend Level 5 for most source code repositories.")
                    .size(11)
                    .color(c.on_surface_variant),
            ]
        )
        .padding(20)
        .width(Length::Fill)
        .style(theme::card_low);

        let right_col = column![
            archive_section,
            Space::with_height(16),
            info_card,
        ]
        .width(Length::FillPortion(5));

        row![left_col, Space::with_width(24), right_col].into()
    }

    fn view_backup_progress(&self) -> Element<'_, Message> {
        let c = self.c();

        if let Some(ref progress) = self.backup_progress {
            let total = progress.total_repos as f32;
            let completed = (progress.completed_repos + progress.failed_repos) as f32;
            let pct = if total > 0.0 { completed / total } else { 0.0 };

            let elapsed_ms = chrono::Utc::now().timestamp_millis() - progress.start_time;
            let elapsed_secs = elapsed_ms / 1000;
            let elapsed_str = format!("{:02}:{:02}:{:02}", elapsed_secs / 3600, (elapsed_secs / 60) % 60, elapsed_secs % 60);

            let remaining_str = if pct > 0.0 && pct < 1.0 {
                let total_est = elapsed_ms as f32 / pct;
                let remaining = (total_est - elapsed_ms as f32) / 1000.0;
                let r = remaining as i64;
                format!("{:02}:{:02}:{:02}", r / 3600, (r / 60) % 60, r % 60)
            } else {
                "---".to_string()
            };

            // Large percentage display
            let pct_display = container(
                column![
                    text("GLOBAL COMPLETION")
                        .size(10)
                        .font(theme::FONT_MONO)
                        .color(c.primary),
                    text(format!("{:.0}%", pct * 100.0))
                        .size(56)
                        .font(theme::FONT_HEADLINE)
                        .color(c.primary_container),
                    Space::with_height(16),
                    row![
                        column![
                            text("Estimated Remaining")
                                .size(11)
                                .color(c.on_surface_variant),
                            text(remaining_str.clone())
                                .size(18)
                                .font(theme::FONT_MONO),
                        ]
                        .spacing(4),
                        Space::with_width(Length::Fill),
                        column![
                            text("Elapsed")
                                .size(11)
                                .color(c.on_surface_variant),
                            text(elapsed_str.clone())
                                .size(18)
                                .font(theme::FONT_MONO)
                                .color(c.secondary),
                        ]
                        .spacing(4),
                    ],
                    Space::with_height(12),
                    progress_bar(0.0..=1.0, pct)
                        .height(6)
                        .style(theme::progress_primary),
                ]
                .spacing(4)
            )
            .padding(32)
            .width(Length::FillPortion(4))
            .style(theme::card_low);

            // Active repos list
            let mut repo_list = column![].spacing(12);
            let accent_colors = [c.tertiary, c.primary, c.secondary];
            for (i, rp) in progress.repos.iter().enumerate() {
                let progress_val = match rp.status.as_str() {
                    "complete" => 1.0,
                    "cloning" => 0.5,
                    _ => 0.0,
                };
                let color_idx = i % accent_colors.len();
                let bar_color = accent_colors[color_idx];

                let status_dot_color = match rp.status.as_str() {
                    "complete" => c.success,
                    "failed" => c.error,
                    "cloning" => bar_color,
                    _ => c.outline,
                };

                let status_text = match rp.status.as_str() {
                    "complete" => "100%".to_string(),
                    "cloning" => "In progress...".to_string(),
                    "failed" => "Failed".to_string(),
                    _ => "Queued".to_string(),
                };

                let error_display: Element<'_, Message> = if let Some(ref err) = rp.error {
                    text(err.as_str())
                        .size(10)
                        .color(c.error)
                        .into()
                } else {
                    Space::with_height(0).into()
                };

                let bar_style = match color_idx {
                    0 => theme::progress_tertiary as fn(&iced::Theme) -> progress_bar::Style,
                    1 => theme::progress_primary,
                    _ => theme::progress_secondary,
                };

                let repo_item = column![
                    row![
                        text("\u{25CF}").size(8).color(status_dot_color),
                        text(&rp.repo_name)
                            .size(13)
                            .font(theme::FONT_MONO),
                        Space::with_width(Length::Fill),
                        text(status_text.clone())
                            .size(11)
                            .font(theme::FONT_MONO)
                            .color(c.on_surface_variant),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                    progress_bar(0.0..=1.0, progress_val as f32)
                        .height(4)
                        .style(bar_style),
                    error_display,
                ]
                .spacing(4);

                repo_list = repo_list.push(repo_item);
            }

            let repos_panel = container(
                column![
                    row![
                        text("Active Repositories")
                            .size(16)
                            .font(theme::FONT_HEADLINE),
                        Space::with_width(Length::Fill),
                        container(
                            text(format!("{} THREADS", self.backup_options.max_concurrent.unwrap_or(3)))
                                .size(10)
                                .font(theme::FONT_MONO)
                                .color(c.secondary)
                        )
                        .padding([4, 8])
                        .style(theme::badge_style(c.secondary)),
                    ]
                    .align_y(Alignment::Center),
                    Space::with_height(16),
                    repo_list,
                ]
            )
            .padding(24)
            .width(Length::FillPortion(8))
            .style(theme::card);

            let current = if let Some(ref repo) = progress.current_repo {
                text(format!("Currently: {}", repo))
                    .size(12)
                    .font(theme::FONT_MONO)
                    .color(c.on_surface_variant)
            } else if !progress.is_running {
                text("Backup complete")
                    .size(12)
                    .font(theme::FONT_MONO)
                    .color(c.success)
            } else {
                text("Starting...")
                    .size(12)
                    .font(theme::FONT_MONO)
                    .color(c.on_surface_variant)
            };

            column![
                current,
                Space::with_height(16),
                row![pct_display, repos_panel].spacing(16),
            ]
            .spacing(0)
            .into()
        } else {
            container(
                column![
                    text("No backup in progress")
                        .size(16)
                        .font(theme::FONT_HEADLINE)
                        .color(c.on_surface_variant),
                    text("Configure options and start a backup to see progress here.")
                        .size(13)
                        .color(c.outline),
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
        let c = self.c();

        if self.backup_history.is_empty() {
            return container(
                column![
                    text("No backup history")
                        .size(16)
                        .font(theme::FONT_HEADLINE)
                        .color(c.on_surface_variant),
                    text("Completed backups will appear here.")
                        .size(13)
                        .color(c.outline),
                ]
                .spacing(8)
            )
            .padding(40)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .into();
        }

        // Table header
        let table_head = container(
            row![
                text("STATUS").size(10).font(theme::FONT_MONO).color(c.outline).width(Length::FillPortion(1)),
                text("TIMESTAMP").size(10).font(theme::FONT_MONO).color(c.outline).width(Length::FillPortion(3)),
                text("REPOS").size(10).font(theme::FONT_MONO).color(c.outline).width(Length::FillPortion(1)),
                text("DURATION").size(10).font(theme::FONT_MONO).color(c.outline).width(Length::FillPortion(2)),
                text("").width(Length::FillPortion(1)),
            ]
            .spacing(8)
            .padding([0, 24])
        )
        .padding([12, 0])
        .width(Length::Fill)
        .style(theme::table_header);

        let mut rows = column![].spacing(0);
        for entry in &self.backup_history {
            let status_color = match entry.status.as_str() {
                "complete" => c.primary,
                "partial" => c.secondary,
                _ => c.error,
            };
            let status_label = match entry.status.as_str() {
                "complete" => "Success",
                "partial" => "Partial",
                _ => "Error",
            };

            let duration_secs = entry.duration / 1000;
            let duration_str = format!("{}m {}s", duration_secs / 60, duration_secs % 60);
            let date_display = entry.date.split('T').next().unwrap_or(&entry.date);

            let entry_row = container(
                row![
                    row![
                        text("\u{25CF}").size(10).color(status_color),
                        text(status_label).size(12).color(status_color),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center)
                    .width(Length::FillPortion(1)),
                    text(date_display)
                        .size(12)
                        .font(theme::FONT_MONO)
                        .color(c.on_surface_variant)
                        .width(Length::FillPortion(3)),
                    text(format!("{}", entry.repo_count))
                        .size(12)
                        .font(theme::FONT_MONO)
                        .width(Length::FillPortion(1)),
                    text(duration_str.clone())
                        .size(12)
                        .font(theme::FONT_MONO)
                        .color(c.on_surface_variant)
                        .width(Length::FillPortion(2)),
                    button(
                        text("Open").size(11).font(theme::FONT_MONO)
                    )
                    .padding([4, 12])
                    .style(theme::ghost_button)
                    .on_press(Message::BackupOpenFolder(entry.destination.clone()))
                    .width(Length::FillPortion(1)),
                ]
                .spacing(8)
                .align_y(Alignment::Center)
                .padding([0, 24])
            )
            .padding([12, 0])
            .width(Length::Fill);

            rows = rows.push(entry_row);
        }

        container(
            column![table_head, rows]
        )
        .width(Length::Fill)
        .style(theme::card)
        .into()
    }
}

fn section_label(label: &str, c: theme::Scheme) -> Element<'_, Message> {
    text(label)
        .size(10)
        .font(theme::FONT_HEADLINE)
        .color(iced::Color { a: 0.8, ..c.primary })
        .into()
}
