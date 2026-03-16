use crate::app::Message;
use crate::types::GitHubRepo;
use iced::widget::{checkbox, column, container, row, text, Space};
use iced::{Element, Length};

pub fn view(repo: &GitHubRepo, is_selected: bool) -> Element<'_, Message> {
    let repo_id = repo.id;

    let header = row![
        checkbox("", is_selected)
            .on_toggle(move |_| Message::RepoToggleSelection(repo_id)),
        text(&repo.name).size(14),
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center);

    let owner_text = text(format!("{}", repo.owner.login))
        .size(11);

    let description = if let Some(ref desc) = repo.description {
        let truncated = if desc.len() > 80 {
            format!("{}...", &desc[..77])
        } else {
            desc.clone()
        };
        text(truncated).size(12)
    } else {
        text("No description").size(12)
    };

    let mut stats_items: Vec<Element<'_, Message>> = Vec::new();

    if let Some(ref lang) = repo.language {
        stats_items.push(text(lang.as_str()).size(11).into());
    }

    stats_items.push(text(format!("\u{2605} {}", repo.stargazers_count)).size(11).into());
    stats_items.push(text(format!("\u{2442} {}", repo.forks_count)).size(11).into());

    if repo.private {
        stats_items.push(Space::with_width(Length::Fill).into());
        stats_items.push(text("Private").size(10).into());
    }

    if repo.fork {
        stats_items.push(text("Fork").size(10).into());
    }

    let stats_row = stats_items.into_iter().fold(
        row![].spacing(12).align_y(iced::Alignment::Center),
        |r, item| r.push(item),
    );

    container(
        column![
            header,
            owner_text,
            description,
            stats_row,
        ]
        .spacing(6)
    )
    .padding(12)
    .width(Length::Fill)
    .style(container::bordered_box)
    .into()
}
