use cursive::theme::{
    BaseColor, BorderStyle, Color, ColorStyle, Palette, PaletteColor, PaletteStyle, Theme,
};

/// Creates a polished, high-contrast retro theme for TTYME.
pub fn create_app_theme() -> Theme {
    let mut theme: Theme = Theme::default();
    theme.shadow = true;
    theme.borders = BorderStyle::Simple;

    let mut palette = Palette::default();
    palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::Primary] = Color::Light(BaseColor::White);
    palette[PaletteColor::Secondary] = Color::Light(BaseColor::White);
    palette[PaletteColor::Tertiary] = Color::Light(BaseColor::Cyan);
    palette[PaletteColor::TitlePrimary] = Color::Light(BaseColor::Cyan);
    palette[PaletteColor::TitleSecondary] = Color::Light(BaseColor::Yellow);
    palette[PaletteColor::Highlight] = Color::Light(BaseColor::Green);
    palette[PaletteColor::HighlightInactive] = Color::Dark(BaseColor::Blue);
    palette[PaletteColor::HighlightText] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::Shadow] = Color::Dark(BaseColor::Black);

    // Explicitly configure editable text styles for maximum readability
    palette[PaletteStyle::EditableText] = ColorStyle::new(
        Color::Light(BaseColor::White),
        Color::Dark(BaseColor::Black),
    )
    .into();
    palette[PaletteStyle::EditableTextCursor] = ColorStyle::new(
        Color::Dark(BaseColor::Black),
        Color::Light(BaseColor::White),
    )
    .into();
    palette[PaletteStyle::EditableTextInactive] = ColorStyle::new(
        Color::Light(BaseColor::White),
        Color::Dark(BaseColor::Black),
    )
    .into();

    theme.palette = palette;
    theme
}
