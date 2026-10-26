use cursive::theme::{BaseColor, BorderStyle, Color, Palette, PaletteColor, Theme};

/// Creates a polished, high-contrast retro theme for TTYME.
pub fn create_app_theme() -> Theme {
    let mut theme = Theme::default();
    theme.shadow = true;
    theme.borders = BorderStyle::Simple;

    let mut palette = Palette::default();
    palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::Primary] = Color::Light(BaseColor::White);
    palette[PaletteColor::TitlePrimary] = Color::Light(BaseColor::Cyan);
    palette[PaletteColor::TitleSecondary] = Color::Light(BaseColor::Yellow);
    palette[PaletteColor::Highlight] = Color::Light(BaseColor::Green);
    palette[PaletteColor::HighlightInactive] = Color::Dark(BaseColor::Blue);
    palette[PaletteColor::Shadow] = Color::Dark(BaseColor::Black);

    theme.palette = palette;
    theme
}
