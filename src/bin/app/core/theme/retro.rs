use cursive::theme::{BorderStyle, Palette, Theme};

pub fn create_retro_theme() -> Theme {
    let mut theme = Theme::retro();
    theme.shadow = true;
    theme.borders = BorderStyle::Simple;
    let palette = Palette::retro();
    theme.palette = palette;
    theme
}
