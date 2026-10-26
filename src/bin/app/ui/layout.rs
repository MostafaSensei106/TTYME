use chrono::Local;
use cursive::{
    Cursive,
    align::HAlign,
    event::{Event, Key},
    theme::{BaseColor, Color, ColorStyle},
    view::{Nameable, Resizable},
    views::{DummyView, EditView, LinearLayout, Panel, ScrollView, TextView},
};

use crate::ui::handlers::handle_submit;
use crate::ui::theme::create_app_theme;

/// Builds and configures the Cursive TUI environment.
pub fn build_ui(username: &str, server_addr: &str) -> Cursive {
    let mut siv = Cursive::new();
    siv.set_theme(create_app_theme());

    // 1. Top Header Banner
    let header_content = format!(
        "⚡ TTYME CHAT  │  👤 User: @{username}  │  🌐 Server: {server_addr}  │  🕒 {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );
    let header = Panel::new(TextView::new(header_content).h_align(HAlign::Center))
        .title(" TTYME v0.1.0 ")
        .title_position(HAlign::Left)
        .full_width();

    let messages_view = TextView::new("").with_name("Messages");

    let messages_panel = Panel::new(
        ScrollView::new(messages_view)
            .scroll_strategy(cursive::view::ScrollStrategy::StickToBottom)
            .min_height(20)
            .full_width(),
    )
    .title(" 💬 Chat Stream ")
    .title_position(HAlign::Center)
    .full_width();

    let prompt = TextView::new(" ❯ ");
    let input = EditView::new()
        .filler(" ")
        .style(ColorStyle::new(
            Color::Light(BaseColor::White),
            Color::Dark(BaseColor::Black),
        ))
        .on_submit(|s, text| handle_submit(s, text.to_string()))
        .with_name("Input")
        .full_width();

    let input_row = LinearLayout::horizontal().child(prompt).child(input);

    let input_panel = Panel::new(input_row)
        .title(" ✏ Message Input (Type here & press Enter) ")
        .title_position(HAlign::Left)
        .full_width();

    // 4. Quick Help / Status Bar Footer
    let footer_text =
        " [ESC] Quit  │  [Enter] Send  │  [/] Quick Command  │  /help  /clear  /quit ";
    let footer = Panel::new(TextView::new(footer_text).h_align(HAlign::Center)).full_width();

    let main_layout = LinearLayout::vertical()
        .child(header)
        .child(DummyView.fixed_height(1))
        .child(messages_panel)
        .child(DummyView.fixed_height(1))
        .child(input_panel)
        .child(DummyView.fixed_height(1))
        .child(footer);

    let centered = LinearLayout::horizontal()
        .child(DummyView.full_width())
        .child(main_layout.min_width(70))
        .child(DummyView.full_width());

    siv.add_fullscreen_layer(centered);

    // Initial Focus on Input
    let _ = siv.focus_name("Input");

    // Global Hotkeys
    siv.add_global_callback(Event::Key(Key::Esc), |s| s.quit());
    siv.add_global_callback('/', |s| {
        let _ = s.call_on_name("Input", |view: &mut EditView| view.set_content("/"));
        let _ = s.focus_name("Input");
    });

    siv
}
