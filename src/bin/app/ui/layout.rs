use chrono::Local;
use cursive::{
    align::HAlign,
    event::{Event, Key},
    theme::{BaseColor, Color, ColorStyle},
    view::{Nameable, Resizable},
    views::{DummyView, EditView, LinearLayout, Panel, ScrollView, TextView},
    Cursive,
};

use crate::ui::handlers::handle_submit;
use crate::ui::theme::create_app_theme;

/// Builds and configures the Cursive TUI environment with pure ASCII styling and widescreen (16:9) layout.
pub fn build_ui(username: &str, server_addr: &str) -> Cursive {
    let mut siv = Cursive::new();
    siv.set_theme(create_app_theme());

    // 1. Top Header Banner with Live Clock Placeholder
    let header_info = format!(" TTYME CHAT  |  User: @{username}  |  Host: {server_addr} ");
    let clock_init = format!("[ {} ]", Local::now().format("%I:%M:%S %p"));
    let clock_view = TextView::new(clock_init).with_name("Clock");

    let header_bar = LinearLayout::horizontal()
        .child(TextView::new(header_info))
        .child(DummyView.full_width())
        .child(clock_view);

    let header = Panel::new(header_bar)
        .title("[ TTYME v0.1.0 ]")
        .title_position(HAlign::Left)
        .full_width();

    // 2. Chat Messages Stream (Dynamic vertical expansion, takes all available space)
    let messages_view = TextView::new("").with_name("Messages");

    let messages_panel = Panel::new(
        ScrollView::new(messages_view)
            .scroll_strategy(cursive::view::ScrollStrategy::StickToBottom)
            .full_width()
            .full_height(),
    )
    .title("[ MESSAGES ]")
    .title_position(HAlign::Center)
    .full_width()
    .full_height();

    // 3. Message Input Field with Prompt Symbol and High-Contrast Styling
    let prompt = TextView::new(" >> ");
    let input = EditView::new()
        .filler(" ")
        .style(ColorStyle::new(
            Color::Light(BaseColor::White),
            Color::Dark(BaseColor::Black),
        ))
        .on_submit(|s, text| handle_submit(s, text.to_string()))
        .with_name("Input")
        .full_width();

    let input_row = LinearLayout::horizontal()
        .child(prompt)
        .child(input);

    let input_panel = Panel::new(input_row)
        .title("[ INPUT ] (Type message and press Enter | / for Commands)")
        .title_position(HAlign::Left)
        .full_width();

    // 4. Quick Help / Shortcuts Footer Bar (Always visible at the bottom)
    let footer_text = " [ESC] Quit  |  [Enter] Send  |  [/] Commands  |  /help  /whoami  /time  /clear  /eggplant  /cucumber  /gigachad  /rip ";
    let footer = Panel::new(TextView::new(footer_text).h_align(HAlign::Center))
        .title("[ SHORTCUTS & COMMANDS ]")
        .title_position(HAlign::Center)
        .full_width();

    // 5. Widescreen Layout (Utilizes the full terminal width & height without clipping)
    let main_layout = LinearLayout::vertical()
        .child(header)
        .child(messages_panel)
        .child(input_panel)
        .child(footer);

    let screen_wrapper = LinearLayout::horizontal()
        .child(DummyView.fixed_width(1))
        .child(main_layout.full_width().full_height())
        .child(DummyView.fixed_width(1));

    siv.add_fullscreen_layer(screen_wrapper);

    // Initial Focus on Input
    let _ = siv.focus_name("Input");

    // Global Keybindings
    siv.add_global_callback(Event::Key(Key::Esc), |s| s.quit());
    siv.add_global_callback('/', |s| {
        let _ = s.call_on_name("Input", |view: &mut EditView| view.set_content("/"));
        let _ = s.focus_name("Input");
    });

    siv
}
