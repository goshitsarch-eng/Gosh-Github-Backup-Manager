use crate::app::{GoshApp, Message};
use crate::types::{GitHubRepo, Page, SortDirection, SortOption, VisibilityFilter};
use crate::widgets::repo_card;
use iced::widget::{button, checkbox, column, container, pick_list, row, scrollable, text, text_input, Space};
use iced::{Alignment, Element, Length};

impl GoshApp {
    pub fn view_repositories(&self) -> Element<'_, Message> {
        let filtered_repos = self.get_filtered_repos();
        let selected_count = self.selected_repos.len();

        // Header
        let header = row![
            column![
                text("Repositories").size(24),
                text(format!("{} total, {} shown", self.repos.len(), filtered_repos.len())).size(13),
            ]
            .spacing(4),
            Space::with_width(Length::Fill),
            if selected_count > 0 {
                Element::from(
                    button(text(format!("Backup ({selected_count})")).size(13))
                        .padding([10, 16])
                        .style(button::primary)
                        .on_press(Message::NavigateTo(Page::Backup))
                )
            } else {
                Element::from(Space::with_width(0))
            },
        ]
        .align_y(Alignment::Center);

        // Filters
        let search = text_input("Search repositories...", &self.repo_search)
            .on_input(Message::RepoSearchChanged)
            .padding(8)
            .size(13)
            .width(Length::FillPortion(2));

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
        .text_size(13);

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
        .text_size(13);

        let sort_dir_label = match self.repo_sort_direction {
            SortDirection::Ascending => "\u{2191}",
            SortDirection::Descending => "\u{2193}",
        };
        let sort_dir_btn = button(text(sort_dir_label).size(14))
            .padding([8, 12])
            .on_press(Message::RepoToggleSortDirection);

        let filter_row = row![
            search,
            visibility_picker,
            sort_picker,
            sort_dir_btn,
        ]
        .spacing(8)
        .align_y(Alignment::Center);

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
        .text_size(13);

        // Owner filter
        let mut owners: Vec<String> = self
            .repos
            .iter()
            .map(|r| r.owner.login.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        owners.sort();
        owners.insert(0, "All Owners".to_string());

        let current_owner = if self.repo_owner_filter.is_empty() {
            Some("All Owners".to_string())
        } else {
            Some(self.repo_owner_filter.clone())
        };

        let owner_picker = pick_list(
            owners,
            current_owner,
            Message::RepoOwnerFilterChanged,
        )
        .padding(8)
        .text_size(13);

        let filter_row2 = row![
            owner_picker,
            lang_picker,
        ]
        .spacing(8);

        // Select all
        let all_filtered_ids: Vec<i64> = filtered_repos.iter().map(|r| r.id).collect();
        let all_selected = !all_filtered_ids.is_empty()
            && all_filtered_ids.iter().all(|id| self.selected_repos.contains(id));

        let select_all = checkbox(
            format!("Select all ({})", filtered_repos.len()),
            all_selected,
        )
        .on_toggle(move |checked| {
            if checked {
                Message::RepoSelectAll
            } else {
                Message::RepoDeselectAll
            }
        })
        .size(16)
        .text_size(13);

        // Repo list
        let mut repo_list = column![].spacing(8);
        for repo in &filtered_repos {
            let is_selected = self.selected_repos.contains(&repo.id);
            repo_list = repo_list.push(repo_card::view(repo, is_selected));
        }

        if filtered_repos.is_empty() {
            repo_list = repo_list.push(
                container(
                    text("No repositories match your filters").size(14)
                )
                .padding(40)
                .width(Length::Fill)
                .center_x(Length::Fill)
            );
        }

        let content = column![
            header,
            Space::with_height(12),
            filter_row,
            filter_row2,
            Space::with_height(8),
            select_all,
            Space::with_height(8),
            repo_list,
            Space::with_height(20),
        ]
        .spacing(4)
        .padding(24);

        scrollable(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn get_filtered_repos(&self) -> Vec<&GitHubRepo> {
        let mut repos: Vec<&GitHubRepo> = self.repos.iter().collect();

        // Search filter
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

        // Owner filter
        if !self.repo_owner_filter.is_empty() {
            repos.retain(|r| r.owner.login == self.repo_owner_filter);
        }

        // Visibility filter
        match self.repo_visibility_filter {
            VisibilityFilter::Public => repos.retain(|r| !r.private),
            VisibilityFilter::Private => repos.retain(|r| r.private),
            VisibilityFilter::All => {}
        }

        // Language filter
        if !self.repo_language_filter.is_empty() {
            repos.retain(|r| {
                r.language
                    .as_ref()
                    .map(|l| l == &self.repo_language_filter)
                    .unwrap_or(false)
            });
        }

        // Sort
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
