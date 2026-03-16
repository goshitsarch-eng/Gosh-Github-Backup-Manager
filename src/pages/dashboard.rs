use crate::app::{GoshApp, Message};
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Alignment, Element, Length};

impl GoshApp {
    pub fn view_dashboard(&self) -> Element<'_, Message> {
        let header = column![
            text("Dashboard").size(24),
            text(format!(
                "Welcome back, {}",
                self.user
                    .as_ref()
                    .and_then(|u| u.name.as_deref())
                    .unwrap_or("User")
            ))
            .size(14),
        ]
        .spacing(4);

        // Profile card
        let profile_card = if let Some(ref user) = self.user {
            let initials = user
                .name
                .as_deref()
                .or(Some(&user.login))
                .unwrap_or("?")
                .chars()
                .next()
                .unwrap_or('?');

            let avatar = container(
                text(format!("{}", initials.to_uppercase())).size(32)
            )
            .width(64)
            .height(64)
            .center_x(64)
            .center_y(64)
            .style(container::bordered_box);

            let user_info = column![
                text(user.name.as_deref().unwrap_or(&user.login)).size(18),
                text(format!("@{}", user.login)).size(12),
                text(format!("{} followers \u{00B7} {} following", user.followers, user.following)).size(12),
            ]
            .spacing(4);

            let bio = if let Some(ref bio) = user.bio {
                text(bio.as_str()).size(12)
            } else {
                text("").size(12)
            };

            let orgs_text = if !self.orgs.is_empty() {
                let org_names: Vec<&str> = self.orgs.iter().map(|o| o.login.as_str()).collect();
                text(format!("Organizations: {}", org_names.join(", "))).size(12)
            } else {
                text("").size(12)
            };

            container(
                column![
                    row![avatar, user_info].spacing(16).align_y(Alignment::Center),
                    bio,
                    orgs_text,
                ]
                .spacing(12)
            )
            .padding(20)
            .width(Length::Fill)
            .style(container::bordered_box)
        } else {
            container(text("Loading profile...").size(14))
                .padding(20)
                .width(Length::Fill)
                .style(container::bordered_box)
        };

        // Stats cards
        let total_repos = self.repos.len() as u32;
        let total_stars: u32 = self.repos.iter().map(|r| r.stargazers_count).sum();
        let total_forks: u32 = self.repos.iter().map(|r| r.forks_count).sum();
        let public_repos = self.repos.iter().filter(|r| !r.private).count() as u32;
        let private_repos = self.repos.iter().filter(|r| r.private).count() as u32;

        let total_repos_str = total_repos.to_string();
        let total_stars_str = total_stars.to_string();
        let total_forks_str = total_forks.to_string();
        let public_repos_str = public_repos.to_string();
        let private_repos_str = private_repos.to_string();
        let orgs_count_str = self.orgs.len().to_string();

        let stats_row1 = row![
            stat_card("Repositories", &total_repos_str),
            stat_card("Stars", &total_stars_str),
            stat_card("Forks", &total_forks_str),
        ]
        .spacing(12);

        let stats_row2 = row![
            stat_card("Public", &public_repos_str),
            stat_card("Private", &private_repos_str),
            stat_card("Organizations", &orgs_count_str),
        ]
        .spacing(12);

        // Quick actions
        let quick_actions = row![
            button(text("View Repositories").size(13))
                .padding([10, 16])
                .style(button::primary)
                .on_press(Message::NavigateTo(crate::types::Page::Repositories)),
            button(text("Start Backup").size(13))
                .padding([10, 16])
                .style(button::secondary)
                .on_press(Message::NavigateTo(crate::types::Page::Backup)),
        ]
        .spacing(12);

        // Recent activity
        let activity_header = text("Recent Activity").size(18);

        let activity_list = if self.events.is_empty() {
            column![text("No recent activity").size(13)]
        } else {
            let mut col = column![].spacing(8);
            for event in self.events.iter().take(10) {
                let event_desc = format_event(event);
                let time_ago = format_time_ago(&event.created_at);
                let event_row = container(
                    row![
                        text(event_desc).size(12),
                        Space::with_width(Length::Fill),
                        text(time_ago).size(11),
                    ]
                    .align_y(Alignment::Center)
                )
                .padding([8, 12])
                .width(Length::Fill)
                .style(container::bordered_box);
                col = col.push(event_row);
            }
            col
        };

        let content = column![
            header,
            Space::with_height(16),
            profile_card,
            Space::with_height(16),
            stats_row1,
            stats_row2,
            Space::with_height(16),
            quick_actions,
            Space::with_height(20),
            activity_header,
            activity_list,
            Space::with_height(20),
        ]
        .spacing(8)
        .padding(24);

        scrollable(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn stat_card(label: &str, value: &str) -> Element<'static, Message> {
    container(
        column![
            text(value.to_string()).size(24),
            text(label.to_string()).size(12),
        ]
        .spacing(4)
        .align_x(Alignment::Center)
    )
    .padding(16)
    .width(Length::Fill)
    .center_x(Length::Fill)
    .style(container::bordered_box)
    .into()
}

fn format_event(event: &crate::types::GitHubEvent) -> String {
    let repo = &event.repo.name;
    match event.event_type.as_str() {
        "PushEvent" => format!("Pushed to {}", repo),
        "CreateEvent" => format!("Created branch/tag in {}", repo),
        "DeleteEvent" => format!("Deleted branch/tag in {}", repo),
        "PullRequestEvent" => format!("PR activity in {}", repo),
        "IssuesEvent" => format!("Issue activity in {}", repo),
        "WatchEvent" => format!("Starred {}", repo),
        "ForkEvent" => format!("Forked {}", repo),
        "IssueCommentEvent" => format!("Commented on issue in {}", repo),
        "PullRequestReviewEvent" => format!("Reviewed PR in {}", repo),
        "ReleaseEvent" => format!("Released in {}", repo),
        other => format!("{} in {}", other, repo),
    }
}

fn format_time_ago(date_str: &str) -> String {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
        let now = chrono::Utc::now();
        let duration = now.signed_duration_since(dt);

        if duration.num_days() > 0 {
            format!("{}d ago", duration.num_days())
        } else if duration.num_hours() > 0 {
            format!("{}h ago", duration.num_hours())
        } else if duration.num_minutes() > 0 {
            format!("{}m ago", duration.num_minutes())
        } else {
            "just now".to_string()
        }
    } else {
        String::new()
    }
}
