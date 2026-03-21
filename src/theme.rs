use iced::widget::{button, container, text_input, progress_bar};
use iced::{Background, Border, Color, Font, Shadow, Theme};
use iced::theme::Palette;

const fn hex(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
}

pub mod colors {
    use iced::Color;
    use super::hex;

    pub const SURFACE: Color = hex(0x0d, 0x13, 0x1e);
    pub const SURFACE_CONTAINER: Color = hex(0x1a, 0x20, 0x2b);
    pub const SURFACE_CONTAINER_HIGH: Color = hex(0x24, 0x2a, 0x36);
    pub const SURFACE_CONTAINER_HIGHEST: Color = hex(0x2f, 0x35, 0x41);
    pub const SURFACE_CONTAINER_LOW: Color = hex(0x16, 0x1c, 0x27);
    pub const SURFACE_CONTAINER_LOWEST: Color = hex(0x08, 0x0e, 0x19);

    pub const PRIMARY: Color = hex(0xa3, 0xdc, 0xec);
    pub const PRIMARY_CONTAINER: Color = hex(0x88, 0xc0, 0xd0);
    pub const ON_PRIMARY: Color = hex(0x00, 0x36, 0x40);

    pub const SECONDARY: Color = hex(0xa9, 0xca, 0xeb);
    pub const TERTIARY: Color = hex(0xf0, 0xc6, 0xe7);
    pub const ON_TERTIARY: Color = hex(0x43, 0x27, 0x40);

    pub const ERROR: Color = hex(0xff, 0xb4, 0xab);
    pub const ERROR_CONTAINER: Color = hex(0x93, 0x00, 0x0a);
    pub const ON_ERROR_CONTAINER: Color = hex(0xff, 0xda, 0xd6);

    pub const ON_SURFACE: Color = hex(0xdd, 0xe2, 0xf2);
    pub const ON_SURFACE_VARIANT: Color = hex(0xc0, 0xc8, 0xcb);

    pub const OUTLINE: Color = hex(0x8a, 0x92, 0x95);
    pub const OUTLINE_VARIANT: Color = hex(0x40, 0x48, 0x4b);

    pub const SUCCESS: Color = hex(0xa3, 0xbe, 0x8c);

    pub const WHITE_5: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.05 };
}

// Font constants
pub const FONT_HEADLINE: Font = Font {
    family: iced::font::Family::Name("Space Grotesk"),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const FONT_HEADLINE_MEDIUM: Font = Font {
    family: iced::font::Family::Name("Space Grotesk"),
    weight: iced::font::Weight::Medium,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const FONT_MONO: Font = Font::with_name("JetBrains Mono");

pub fn gitsafe_theme() -> Theme {
    Theme::custom("GitSafe".to_string(), Palette {
        background: colors::SURFACE,
        text: colors::ON_SURFACE,
        primary: colors::PRIMARY,
        success: colors::SUCCESS,
        danger: colors::ERROR,
    })
}

// ---- Container Styles ----

pub fn sidebar(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::SURFACE_CONTAINER_LOW)),
        ..Default::default()
    }
}

pub fn header(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::SURFACE)),
        border: Border {
            color: colors::WHITE_5,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::SURFACE_CONTAINER)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn card_low(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::SURFACE_CONTAINER_LOW)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn card_lowest(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::SURFACE_CONTAINER_LOWEST)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn card_high(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::SURFACE_CONTAINER_HIGH)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn card_highest(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::SURFACE_CONTAINER_HIGHEST)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn accent_bar(color: Color) -> impl Fn(&Theme) -> container::Style {
    move |_theme: &Theme| container::Style {
        background: Some(Background::Color(color)),
        ..Default::default()
    }
}

pub fn status_badge(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color { a: 0.1, ..colors::PRIMARY })),
        border: Border {
            color: Color { a: 0.2, ..colors::PRIMARY },
            width: 1.0,
            radius: 12.0.into(),
        },
        ..Default::default()
    }
}

pub fn badge_style(color: Color) -> impl Fn(&Theme) -> container::Style {
    move |_theme: &Theme| container::Style {
        background: Some(Background::Color(Color { a: 0.1, ..color })),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 12.0.into(),
        },
        ..Default::default()
    }
}

pub fn table_header(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::SURFACE_CONTAINER_HIGH)),
        ..Default::default()
    }
}

pub fn input_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::SURFACE_CONTAINER_LOWEST)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 2.0.into(),
        },
        ..Default::default()
    }
}

// ---- Button Styles ----

pub fn primary_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(colors::PRIMARY)),
        text_color: colors::ON_PRIMARY,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 2.0.into(),
        },
        shadow: Shadow::default(),
    };
    match status {
        button::Status::Hovered | button::Status::Pressed => button::Style {
            background: Some(Background::Color(colors::PRIMARY_CONTAINER)),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color { a: 0.3, ..colors::PRIMARY })),
            text_color: Color { a: 0.5, ..colors::ON_PRIMARY },
            ..base
        },
        _ => base,
    }
}

pub fn ghost_button(_theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        background: match status {
            button::Status::Hovered | button::Status::Pressed => {
                Some(Background::Color(colors::SURFACE_CONTAINER_HIGHEST))
            }
            _ => None,
        },
        text_color: match status {
            button::Status::Hovered => colors::ON_SURFACE,
            _ => colors::ON_SURFACE_VARIANT,
        },
        border: Border::default(),
        shadow: Shadow::default(),
    }
}

pub fn danger_button(_theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered | button::Status::Pressed => colors::ERROR,
            _ => colors::ERROR_CONTAINER,
        })),
        text_color: colors::ON_ERROR_CONTAINER,
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
        shadow: Shadow::default(),
    }
}

pub fn nav_active(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: Some(Background::Color(colors::SURFACE_CONTAINER_HIGHEST)),
        text_color: colors::PRIMARY,
        border: Border::default(),
        shadow: Shadow::default(),
    }
}

pub fn nav_inactive(_theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        background: match status {
            button::Status::Hovered => Some(Background::Color(colors::SURFACE_CONTAINER_HIGH)),
            _ => None,
        },
        text_color: match status {
            button::Status::Hovered => colors::ON_SURFACE,
            _ => colors::ON_SURFACE_VARIANT,
        },
        border: Border::default(),
        shadow: Shadow::default(),
    }
}

pub fn tab_active(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: Some(Background::Color(colors::SURFACE_CONTAINER_HIGHEST)),
        text_color: colors::PRIMARY,
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
        shadow: Shadow::default(),
    }
}

pub fn tab_inactive(_theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        background: match status {
            button::Status::Hovered => Some(Background::Color(colors::SURFACE_CONTAINER_HIGH)),
            _ => Some(Background::Color(colors::SURFACE_CONTAINER_LOWEST)),
        },
        text_color: match status {
            button::Status::Hovered => colors::ON_SURFACE,
            _ => colors::ON_SURFACE_VARIANT,
        },
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
        shadow: Shadow::default(),
    }
}

pub fn tertiary_button(_theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered | button::Status::Pressed => {
                Color { a: 0.85, ..colors::TERTIARY }
            }
            _ => colors::TERTIARY,
        })),
        text_color: colors::ON_TERTIARY,
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
        shadow: Shadow::default(),
    }
}

pub fn mode_card_active(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: Some(Background::Color(colors::SURFACE_CONTAINER)),
        text_color: colors::ON_SURFACE,
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
        shadow: Shadow::default(),
    }
}

pub fn mode_card_inactive(_theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered => colors::SURFACE_CONTAINER_HIGH,
            _ => colors::SURFACE_CONTAINER_LOW,
        })),
        text_color: match status {
            button::Status::Hovered => colors::ON_SURFACE,
            _ => colors::ON_SURFACE_VARIANT,
        },
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
        shadow: Shadow::default(),
    }
}

// ---- Text Input Style ----

pub fn surface_input(_theme: &Theme, status: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: Background::Color(colors::SURFACE_CONTAINER_LOWEST),
        border: Border {
            color: match status {
                text_input::Status::Focused => Color { a: 0.3, ..colors::PRIMARY },
                text_input::Status::Hovered => colors::OUTLINE_VARIANT,
                _ => Color::TRANSPARENT,
            },
            width: match status {
                text_input::Status::Focused => 1.0,
                text_input::Status::Hovered => 1.0,
                _ => 0.0,
            },
            radius: 2.0.into(),
        },
        icon: colors::ON_SURFACE_VARIANT,
        placeholder: colors::OUTLINE,
        value: colors::PRIMARY,
        selection: Color { a: 0.3, ..colors::PRIMARY },
    }
}

// ---- Progress Bar Style ----

pub fn progress_primary(_theme: &Theme) -> progress_bar::Style {
    progress_bar::Style {
        background: Background::Color(colors::SURFACE_CONTAINER_HIGHEST),
        bar: Background::Color(colors::PRIMARY),
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
    }
}

pub fn progress_tertiary(_theme: &Theme) -> progress_bar::Style {
    progress_bar::Style {
        background: Background::Color(colors::SURFACE_CONTAINER_HIGHEST),
        bar: Background::Color(colors::TERTIARY),
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
    }
}

pub fn progress_secondary(_theme: &Theme) -> progress_bar::Style {
    progress_bar::Style {
        background: Background::Color(colors::SURFACE_CONTAINER_HIGHEST),
        bar: Background::Color(colors::SECONDARY),
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
    }
}

// ---- Language Colors ----

pub fn language_color(lang: &str) -> Color {
    match lang {
        "Rust" => hex(0xde, 0xa5, 0x84),
        "JavaScript" => hex(0xf1, 0xe0, 0x5a),
        "TypeScript" => hex(0x31, 0x78, 0xc6),
        "Python" => hex(0x35, 0x72, 0xa5),
        "Go" => hex(0x00, 0xad, 0xd8),
        "Java" => hex(0xb0, 0x72, 0x19),
        "C" => hex(0x55, 0x55, 0x55),
        "C++" => hex(0xf3, 0x4b, 0x7d),
        "C#" => hex(0x17, 0x8e, 0x00),
        "Ruby" => hex(0x70, 0x11, 0x16),
        "PHP" => hex(0x4f, 0x5d, 0x95),
        "Swift" => hex(0xf0, 0x52, 0x38),
        "Kotlin" => hex(0xa9, 0x7b, 0xff),
        "Shell" | "Bash" => hex(0x89, 0xe0, 0x51),
        "HTML" => hex(0xe3, 0x4c, 0x26),
        "CSS" => hex(0x56, 0x3d, 0x7c),
        "Dart" => hex(0x00, 0xb4, 0xab),
        "Lua" => hex(0x00, 0x00, 0x80),
        "Zig" => hex(0xec, 0x91, 0x5c),
        "Haskell" => hex(0x5e, 0x50, 0x86),
        "Elixir" => hex(0x6e, 0x40, 0x94),
        "Scala" => hex(0xc2, 0x25, 0x35),
        "Vue" => hex(0x41, 0xb8, 0x83),
        _ => colors::ON_SURFACE_VARIANT,
    }
}
