use crate::app::{GoshApp, Message};
use crate::theme;
use crate::types::{GitHubRepo, Page, SortDirection, SortOption, VisibilityFilter};
use iced::widget::{button, checkbox, column, container, pick_list, row, scrollable, text, text_input, Space};
use iced::{Alignment, Element, Length};

impl GoshApp {
    pub fn view_repositories(&self) -> Element<'_, Message> {
        let filtered_repos = self.get_filtered_repos();
        let selected_count = self.selected_repos.len();

        // ---- Header Section ----
        let header_left = {
            container(
                column![
                    text("Repository Selection")
                        .size(22)
                        .font(theme::FONT_HEADLINE),
                    text("Choose the repositories you wish to backup. Selected repositories will be backed up according to your settings.")
                        .size(13)
                        .color(theme::colors::ON_SURFACE_VARIANT),
                    Space::with_height(16),
                    row![
                        stat_badge("TOTAL DETECTED", self.repos.len().to_string(), theme::colors::PRIMARY),
                        stat_badge("SELECTED", selected_count.to_string(), theme::colors::TERTIARY),
                    ]
                    .spacing(32),
                ]
                .spacing(8)
            )
            .padding(24)
            .width(Length::FillPortion(8))
            .style(theme::card_low)
        };

        let header_right = container(
            column![
                text("\u{2728}")
                    .size(20)
                    .color(theme::colors::TERTIARY),
                Space::with_height(8),
                text("Smart Selection")
                    .size(14)
                    .font(theme::FONT_HEADLINE),
                text("Automatically select all repositories pushed in the last 30 days.")
                    .size(11)
                    .color(theme::colors::ON_SURFACE_VARIANT),
                Space::with_height(8),
                button(
                    text("SELECT ALL")
                        .size(10)
                        .font(theme::FONT_MONO)
                        .color(theme::colors::PRIMARY)
                )
                .padding([6, 0])
                .style(theme::ghost_button)
                .on_press(Message::RepoSelectAll),
            ]
            .spacing(4)
        )
        .padding(24)
        .width(Length::FillPortion(4))
        .style(theme::card_highest);

        let header_row = row![header_left, header_right].spacing(16);

        // ---- Filter Bar ----
        let search = text_input("Search repository name, tag, or owner...", &self.repo_search)
            .on_input(Message::RepoSearchChanged)
            .padding(10)
            .size(13)
            .style(theme::surface_input)
            .width(Length::FillPortion(3));

        let visibility_options = vec![
            VisibilityFilter::All,
            VisibilityFilter::Public,
            VisibilityFilter::Private,
        ];
        let visibility_picker = pick_list(
            visibility_options,
            Some(self.repo_visibility_filter),
            Message::RepoVisibilityFilterChanged,
        )
        .padding(8)
        .text_size(12);

        // Language filter
        let mut languages: Vec<String> = self
            .repos
            .iter()
            .filter_map(|r| r.language.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        languages.sort();
        languages.insert(0, "All Languages".to_string());
        let current_lang = if self.repo_language_filter.is_empty() {
            Some("All Languages".to_string())
        } else {
            Some(self.repo_language_filter.clone())
        };
        let lang_picker = pick_list(
            languages,
            current_lang,
            Message::RepoLanguageFilterChanged,
        )
        .padding(8)
        .text_size(12);

        let sort_options = vec![
            SortOption::Name,
            SortOption::Stars,
            SortOption::Updated,
            SortOption::Size,
        ];
        let sort_picker = pick_list(
            sort_options,
            Some(self.repo_sort_by),
            Message::RepoSortByChanged,
        )
        .padding(8)
        .text_size(12);

        let sort_dir_label = match self.repo_sort_direction {
            SortDirection::Ascending => "\u{2191}",
            SortDirection::Descending => "\u{2193}",
        };
        let sort_dir_btn = button(text(sort_dir_label).size(13))
            .padding([8, 12])
            .style(theme::ghost_button)
            .on_press(Message::RepoToggleSortDirection);

        let select_all_btn = button(
            text("Select All").size(10).font(theme::FONT_MONO)
        )
        .padding([6, 8])
        .style(theme::ghost_button)
        .on_press(Message::RepoSelectAll);

        let deselect_btn = button(
            text("Deselect All").size(10).font(theme::FONT_MONO)
        )
        .padding([6, 8])
        .style(theme::ghost_button)
        .on_press(Message::RepoDeselectAll);

        let configure_btn = if selected_count > 0 {
            Element::from(
                button(
                    text(format!("Configure Backup ({})", selected_count))
                        .size(11)
                        .font(theme::FONT_HEADLINE)
                )
                .padding([8, 16])
                .style(theme::tertiary_button)
                .on_press(Message::NavigateTo(Page::Backup))
            )
        } else {
            Element::from(Space::with_width(0))
        };

        let filter_bar = container(
            row![
                search,
                visibility_picker,
                lang_picker,
                sort_picker,
                sort_dir_btn,
                Space::with_width(Length::Fill),
                select_all_btn,
                deselect_btn,
                configure_btn,
            ]
            .spacing(8)
            .align_y(Alignment::Center)
        )
        .padding(16)
        .width(Length::Fill)
        .style(theme::card);

        // ---- Repository Table ----
        let table_head = container(
            row![
                Space::with_width(40),
                text("REPOSITORY NAME")
                    .size(10).font(theme::FONT_MONO).color(theme::colors::ON_SURFACE_VARIANT)
                    .width(Length::FillPortion(4)),
                text("VISIBILITY")
                    .size(10).font(theme::FONT_MONO).color(theme::colors::ON_SURFACE_VARIANT)
                    .width(Length::FillPortion(2)),
                text("LANGUAGE")
                    .size(10).font(theme::FONT_MONO).color(theme::colors::ON_SURFACE_VARIANT)
                    .width(Length::FillPortion(2)),
                text("STARS")
                    .size(10).font(theme::FONT_MONO).color(theme::colors::ON_SURFACE_VARIANT)
                    .width(Length::FillPortion(1)),
                text("SIZE")
                    .size(10).font(theme::FONT_MONO).color(theme::colors::ON_SURFACE_VARIANT)
                    .width(Length::FillPortion(1)),
            ]
            .spacing(8)
            .align_y(Alignment::Center)
            .padding([0, 24])
        )
        .padding([12, 0])
        .width(Length::Fill)
        .style(theme::table_header);

        let mut table_rows = column![].spacing(0);
        for repo in &filtered_repos {
            let is_selected = self.selected_repos.contains(&repo.id);
            table_rows = table_rows.push(repo_table_row(repo, is_selected));
        }

        if filtered_repos.is_empty() {
            table_rows = table_rows.push(
                container(
                    text("No repositories match your filters")
                        .size(13)
                        .color(theme::colors::ON_SURFACE_VARIANT)
                )
                .padding(40)
                .width(Length::Fill)
                .center_x(Length::Fill)
            );
        }

        // Footer
        let footer = container(
            text(format!("Showing {} of {} repositories", filtered_repos.len(), self.repos.len()))
                .size(11)
                .font(theme::FONT_MONO)
                .color(theme::colors::OUTLINE)
        )
        .padding([12, 24])
        .width(Length::Fill)
        .style(theme::card_lowest);

        let table = container(
            column![table_head, table_rows, footer]
        )
        .width(Length::Fill)
        .style(theme::card_low);

        // ---- Full Layout ----
        let content = column![
            header_row,
            Space::with_height(16),
            filter_bar,
            Space::with_height(16),
            table,
            Space::with_height(24),
        ]
        .spacing(0)
        .padding(32);

        scrollable(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn get_filtered_repos(&self) -> Vec<&GitHubRepo> {
        let mut repos: Vec<&GitHubRepo> = self.repos.iter().collect();

        if !self.repo_search.is_empty() {
            let search = self.repo_search.to_lowercase();
            repos.retain(|r| {
                r.name.to_lowercase().contains(&search)
                    || r.full_name.to_lowercase().contains(&search)
                    || r.description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&search))
                        .unwrap_or(false)
            });
        }

        if !self.repo_owner_filter.is_empty() {
            repos.retain(|r| r.owner.login == self.repo_owner_filter);
        }

        match self.repo_visibility_filter {
            VisibilityFilter::Public => repos.retain(|r| !r.private),
            VisibilityFilter::Private => repos.retain(|r| r.private),
            VisibilityFilter::All => {}
        }

        if !self.repo_language_filter.is_empty() {
            repos.retain(|r| {
                r.language
                    .as_ref()
                    .map(|l| l == &self.repo_language_filter)
                    .unwrap_or(false)
            });
        }

        repos.sort_by(|a, b| {
            let cmp = match self.repo_sort_by {
                SortOption::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                SortOption::Stars => a.stargazers_count.cmp(&b.stargazers_count),
                SortOption::Updated => a.updated_at.cmp(&b.updated_at),
                SortOption::Size => a.size.cmp(&b.size),
            };
            match self.repo_sort_direction {
                SortDirection::Ascending => cmp,
                SortDirection::Descending => cmp.reverse(),
            }
        });

        repos
    }
}

fn stat_badge<'a>(label: &'a str, value: String, color: iced::Color) -> Element<'a, Message> {
    column![
        text(label)
            .size(10)
            .font(theme::FONT_MONO)
            .color(theme::colors::OUTLINE),
        text(value)
            .size(18)
            .font(theme::FONT_HEADLINE)
            .color(color),
    ]
    .spacing(4)
    .into()
}

fn repo_table_row<'a>(repo: &'a GitHubRepo, is_selected: bool) -> Element<'a, Message> {
    let repo_id = repo.id;

    // Visibility badge
    let vis_badge = if repo.private {
        container(
            text("Private")
                .size(10)
                .font(theme::FONT_MONO)
                .color(theme::colors::SECONDARY)
        )
        .padding([3, 8])
        .style(theme::badge_style(theme::colors::SECONDARY))
    } else {
        container(
            text("Public")
                .size(10)
                .font(theme::FONT_MONO)
                .color(theme::colors::ON_SURFACE_VARIANT)
        )
        .padding([3, 8])
        .style(theme::card_highest)
    };

    // Language with color dot
    let lang_display: Element<'a, Message> = if let Some(ref lang) = repo.language {
        let lang_color = theme::language_color(lang);
        row![
            text("\u{25CF}").size(10).color(lang_color),
            text(lang.as_str()).size(11).color(theme::colors::ON_SURFACE_VARIANT),
        ]
        .spacing(6)
        .align_y(Alignment::Center)
        .into()
    } else {
        text("\u{2014}")
            .size(11)
            .color(theme::colors::OUTLINE_VARIANT)
            .into()
    };

    // Size formatting
    let size_str = if repo.size >= 1024 {
        format!("{:.1} GB", repo.size as f64 / 1024.0)
    } else {
        format!("{} MB", repo.size)
    };

    container(
        row![
            checkbox("", is_selected)
                .on_toggle(move |_| Message::RepoToggleSelection(repo_id))
                .size(16),
            column![
                text(&repo.name)
                    .size(13)
                    .color(theme::colors::ON_SURFACE),
                text(&repo.full_name)
                    .size(10)
                    .font(theme::FONT_MONO)
                    .color(theme::colors::OUTLINE),
            ]
            .spacing(2)
            .width(Length::FillPortion(4)),
            container(vis_badge).width(Length::FillPortion(2)),
            container(lang_display).width(Length::FillPortion(2)),
            text(format!("\u{2605} {}", repo.stargazers_count))
                .size(11)
                .font(theme::FONT_MONO)
                .color(theme::colors::ON_SURFACE_VARIANT)
                .width(Length::FillPortion(1)),
            text(size_str)
                .size(11)
                .font(theme::FONT_MONO)
                .color(theme::colors::OUTLINE)
                .width(Length::FillPortion(1)),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .padding([0, 24])
    )
    .padding([12, 0])
    .width(Length::Fill)
    .into()
}
