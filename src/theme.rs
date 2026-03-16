use crate::types::AppTheme;
use iced::Theme;

pub fn get_theme(app_theme: AppTheme) -> Theme {
    match app_theme {
        AppTheme::Light => Theme::Light,
        AppTheme::Dark => Theme::Dark,
        AppTheme::System => Theme::Dark, // Default to dark for now
    }
}

// Color constants for consistent styling
pub mod colors {
    use iced::Color;

    pub const GREEN: Color = Color::from_rgb(0.137, 0.525, 0.212); // #238636
    pub const RED: Color = Color::from_rgb(0.847, 0.227, 0.227);
    pub const BLUE: Color = Color::from_rgb(0.345, 0.584, 0.929);
    pub const YELLOW: Color = Color::from_rgb(0.886, 0.698, 0.094);
    pub const MUTED: Color = Color::from_rgb(0.55, 0.55, 0.55);
    pub const SIDEBAR_BG: Color = Color::from_rgb(0.08, 0.08, 0.12);
    pub const CARD_BG: Color = Color::from_rgb(0.12, 0.12, 0.16);
    pub const SURFACE: Color = Color::from_rgb(0.15, 0.15, 0.19);
}
