use crate::types::AppTheme;
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

// ---- Scheme ----

#[derive(Debug, Clone, Copy)]
pub struct Scheme {
    pub surface: Color,
    pub surface_container: Color,
    pub surface_container_high: Color,
    pub surface_container_highest: Color,
    pub surface_container_low: Color,
    pub surface_container_lowest: Color,
    pub primary: Color,
    pub primary_container: Color,
    pub on_primary: Color,
    pub secondary: Color,
    pub tertiary: Color,
    pub on_tertiary: Color,
    pub error: Color,
    pub error_container: Color,
    pub on_error_container: Color,
    pub on_surface: Color,
    pub on_surface_variant: Color,
    pub outline: Color,
    pub outline_variant: Color,
    pub success: Color,
    pub border_subtle: Color,
}

pub const DARK: Scheme = Scheme {
    surface: hex(0x0d, 0x13, 0x1e),
    surface_container: hex(0x1a, 0x20, 0x2b),
    surface_container_high: hex(0x24, 0x2a, 0x36),
    surface_container_highest: hex(0x2f, 0x35, 0x41),
    surface_container_low: hex(0x16, 0x1c, 0x27),
    surface_container_lowest: hex(0x08, 0x0e, 0x19),
    primary: hex(0xa3, 0xdc, 0xec),
    primary_container: hex(0x88, 0xc0, 0xd0),
    on_primary: hex(0x00, 0x36, 0x40),
    secondary: hex(0xa9, 0xca, 0xeb),
    tertiary: hex(0xf0, 0xc6, 0xe7),
    on_tertiary: hex(0x43, 0x27, 0x40),
    error: hex(0xff, 0xb4, 0xab),
    error_container: hex(0x93, 0x00, 0x0a),
    on_error_container: hex(0xff, 0xda, 0xd6),
    on_surface: hex(0xdd, 0xe2, 0xf2),
    on_surface_variant: hex(0xc0, 0xc8, 0xcb),
    outline: hex(0x8a, 0x92, 0x95),
    outline_variant: hex(0x40, 0x48, 0x4b),
    success: hex(0xa3, 0xbe, 0x8c),
    border_subtle: Color { r: 1.0, g: 1.0, b: 1.0, a: 0.05 },
};

pub const LIGHT: Scheme = Scheme {
    surface: hex(0xf8, 0xf9, 0xfc),
    surface_container: hex(0xee, 0xf0, 0xf5),
    surface_container_high: hex(0xe8, 0xea, 0xf0),
    surface_container_highest: hex(0xe2, 0xe4, 0xea),
    surface_container_low: hex(0xf3, 0xf5, 0xfa),
    surface_container_lowest: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
    primary: hex(0x2b, 0x66, 0x74),
    primary_container: hex(0xb3, 0xec, 0xfc),
    on_primary: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
    secondary: hex(0x29, 0x49, 0x65),
    tertiary: hex(0x5b, 0x3d, 0x57),
    on_tertiary: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
    error: hex(0xba, 0x1a, 0x1a),
    error_container: hex(0xff, 0xda, 0xd6),
    on_error_container: hex(0x41, 0x00, 0x02),
    on_surface: hex(0x1a, 0x1c, 0x1e),
    on_surface_variant: hex(0x42, 0x47, 0x4e),
    outline: hex(0x73, 0x77, 0x7f),
    outline_variant: hex(0xc3, 0xc7, 0xcf),
    success: hex(0x4a, 0x67, 0x41),
    border_subtle: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.08 },
};

pub fn scheme(app_theme: AppTheme) -> Scheme {
    match app_theme {
        AppTheme::Dark | AppTheme::System => DARK,
        AppTheme::Light => LIGHT,
    }
}

fn is_dark(theme: &Theme) -> bool {
    let bg = theme.palette().background;
    (bg.r + bg.g + bg.b) / 3.0 < 0.5
}

fn s(theme: &Theme) -> Scheme {
    if is_dark(theme) { DARK } else { LIGHT }
}

pub fn gitsafe_theme(app_theme: AppTheme) -> Theme {
    let sc = scheme(app_theme);
    Theme::custom("GitSafe".to_string(), Palette {
        background: sc.surface,
        text: sc.on_surface,
        primary: sc.primary,
        success: sc.success,
        danger: sc.error,
    })
}

// Backward compat: keep colors module
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

// ---- Container Styles ----

pub fn sidebar(theme: &Theme) -> container::Style {
    let c = s(theme);
    container::Style {
        background: Some(Background::Color(c.surface_container_low)),
        ..Default::default()
    }
}

pub fn header(theme: &Theme) -> container::Style {
    let c = s(theme);
    container::Style {
        background: Some(Background::Color(c.surface)),
        border: Border {
            color: c.border_subtle,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn card(theme: &Theme) -> container::Style {
    let c = s(theme);
    container::Style {
        background: Some(Background::Color(c.surface_container)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn card_low(theme: &Theme) -> container::Style {
    let c = s(theme);
    container::Style {
        background: Some(Background::Color(c.surface_container_low)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn card_lowest(theme: &Theme) -> container::Style {
    let c = s(theme);
    container::Style {
        background: Some(Background::Color(c.surface_container_lowest)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn card_high(theme: &Theme) -> container::Style {
    let c = s(theme);
    container::Style {
        background: Some(Background::Color(c.surface_container_high)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn card_highest(theme: &Theme) -> container::Style {
    let c = s(theme);
    container::Style {
        background: Some(Background::Color(c.surface_container_highest)),
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

pub fn status_badge(theme: &Theme) -> container::Style {
    let c = s(theme);
    container::Style {
        background: Some(Background::Color(Color { a: 0.1, ..c.primary })),
        border: Border {
            color: Color { a: 0.2, ..c.primary },
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

pub fn table_header(theme: &Theme) -> container::Style {
    let c = s(theme);
    container::Style {
        background: Some(Background::Color(c.surface_container_high)),
        ..Default::default()
    }
}

pub fn input_container(theme: &Theme) -> container::Style {
    let c = s(theme);
    container::Style {
        background: Some(Background::Color(c.surface_container_lowest)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 2.0.into(),
        },
        ..Default::default()
    }
}

// ---- Button Styles ----

pub fn primary_button(theme: &Theme, status: button::Status) -> button::Style {
    let c = s(theme);
    let base = button::Style {
        background: Some(Background::Color(c.primary)),
        text_color: c.on_primary,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 2.0.into(),
        },
        shadow: Shadow::default(),
    };
    match status {
        button::Status::Hovered | button::Status::Pressed => button::Style {
            background: Some(Background::Color(c.primary_container)),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color { a: 0.3, ..c.primary })),
            text_color: Color { a: 0.5, ..c.on_primary },
            ..base
        },
        _ => base,
    }
}

pub fn ghost_button(theme: &Theme, status: button::Status) -> button::Style {
    let c = s(theme);
    button::Style {
        background: match status {
            button::Status::Hovered | button::Status::Pressed => {
                Some(Background::Color(c.surface_container_highest))
            }
            _ => None,
        },
        text_color: match status {
            button::Status::Hovered => c.on_surface,
            _ => c.on_surface_variant,
        },
        border: Border::default(),
        shadow: Shadow::default(),
    }
}

pub fn danger_button(theme: &Theme, status: button::Status) -> button::Style {
    let c = s(theme);
    button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered | button::Status::Pressed => c.error,
            _ => c.error_container,
        })),
        text_color: c.on_error_container,
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
        shadow: Shadow::default(),
    }
}

pub fn nav_active(theme: &Theme, _status: button::Status) -> button::Style {
    let c = s(theme);
    button::Style {
        background: Some(Background::Color(c.surface_container_highest)),
        text_color: c.primary,
        border: Border::default(),
        shadow: Shadow::default(),
    }
}

pub fn nav_inactive(theme: &Theme, status: button::Status) -> button::Style {
    let c = s(theme);
    button::Style {
        background: match status {
            button::Status::Hovered => Some(Background::Color(c.surface_container_high)),
            _ => None,
        },
        text_color: match status {
            button::Status::Hovered => c.on_surface,
            _ => c.on_surface_variant,
        },
        border: Border::default(),
        shadow: Shadow::default(),
    }
}

pub fn tab_active(theme: &Theme, _status: button::Status) -> button::Style {
    let c = s(theme);
    button::Style {
        background: Some(Background::Color(c.surface_container_highest)),
        text_color: c.primary,
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
        shadow: Shadow::default(),
    }
}

pub fn tab_inactive(theme: &Theme, status: button::Status) -> button::Style {
    let c = s(theme);
    button::Style {
        background: match status {
            button::Status::Hovered => Some(Background::Color(c.surface_container_high)),
            _ => Some(Background::Color(c.surface_container_lowest)),
        },
        text_color: match status {
            button::Status::Hovered => c.on_surface,
            _ => c.on_surface_variant,
        },
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
        shadow: Shadow::default(),
    }
}

pub fn tertiary_button(theme: &Theme, status: button::Status) -> button::Style {
    let c = s(theme);
    button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered | button::Status::Pressed => {
                Color { a: 0.85, ..c.tertiary }
            }
            _ => c.tertiary,
        })),
        text_color: c.on_tertiary,
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
        shadow: Shadow::default(),
    }
}

pub fn mode_card_active(theme: &Theme, _status: button::Status) -> button::Style {
    let c = s(theme);
    button::Style {
        background: Some(Background::Color(c.surface_container)),
        text_color: c.on_surface,
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
        shadow: Shadow::default(),
    }
}

pub fn mode_card_inactive(theme: &Theme, status: button::Status) -> button::Style {
    let c = s(theme);
    button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered => c.surface_container_high,
            _ => c.surface_container_low,
        })),
        text_color: match status {
            button::Status::Hovered => c.on_surface,
            _ => c.on_surface_variant,
        },
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
        shadow: Shadow::default(),
    }
}

// ---- Text Input Style ----

pub fn surface_input(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let c = s(theme);
    text_input::Style {
        background: Background::Color(c.surface_container_lowest),
        border: Border {
            color: match status {
                text_input::Status::Focused => Color { a: 0.3, ..c.primary },
                text_input::Status::Hovered => c.outline_variant,
                _ => Color::TRANSPARENT,
            },
            width: match status {
                text_input::Status::Focused => 1.0,
                text_input::Status::Hovered => 1.0,
                _ => 0.0,
            },
            radius: 2.0.into(),
        },
        icon: c.on_surface_variant,
        placeholder: c.outline,
        value: c.primary,
        selection: Color { a: 0.3, ..c.primary },
    }
}

// ---- Progress Bar Style ----

pub fn progress_primary(theme: &Theme) -> progress_bar::Style {
    let c = s(theme);
    progress_bar::Style {
        background: Background::Color(c.surface_container_highest),
        bar: Background::Color(c.primary),
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
    }
}

pub fn progress_tertiary(theme: &Theme) -> progress_bar::Style {
    let c = s(theme);
    progress_bar::Style {
        background: Background::Color(c.surface_container_highest),
        bar: Background::Color(c.tertiary),
        border: Border {
            radius: 2.0.into(),
            ..Default::default()
        },
    }
}

pub fn progress_secondary(theme: &Theme) -> progress_bar::Style {
    let c = s(theme);
    progress_bar::Style {
        background: Background::Color(c.surface_container_highest),
        bar: Background::Color(c.secondary),
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
