use std::{env, error::Error, sync::Arc};

use chrono::Local;
use cursive::{
    Cursive,
    align::HAlign,
    event::{Event, Key},
    view::{Nameable, Resizable, Scrollable},
    views::{Dialog, DummyView, EditView, LinearLayout, Panel, ScrollView, TextView},
};
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex};

use crate::core::theme::retro::create_retro_theme;
mod core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let username = env::args()
        .nth(1)
        .expect("Please provide a user aneame as argument");

    // ui init
    let mut siv = cursive::default();
    siv.set_theme(create_retro_theme());

    let header = TextView::new(format!(
        r#"
            •{username},{} •
        "#,
        Local::now().format("%Y-%m-%d %H:%M:%S")
    ))
    .h_align(HAlign::Center);

    // TEXT
    let messages = TextView::new("")
        .with_name("Messages")
        .min_height(20)
        .scrollable();
    let messages = ScrollView::new(messages)
        .scroll_strategy(cursive::view::ScrollStrategy::StickToBottom)
        .min_height(60)
        .full_width();

    let input = EditView::new()
        .on_submit(move |s, text| send_messages(s, text.to_string()))
        .with_name("Input")
        .max_height(3)
        .min_width(50)
        .full_width();

    let help_text = TextView::new("ESC:quit | Enter:send |comands: /help, /clear, /quit");

    let layout = LinearLayout::vertical()
        .child(Panel::new(header))
        .child(
            Dialog::around(messages)
                .title("Messages")
                .title_position(HAlign::Center)
                .full_width(),
        )
        .child(
            Dialog::around(input)
                .title("Message")
                .title_position(HAlign::Center)
                .full_width(),
        )
        .child(Panel::new(help_text).full_width());
    let center_layout = LinearLayout::horizontal()
        .child(DummyView.full_width())
        .child(layout)
        .child(DummyView.full_width());

    siv.add_fullscreen_layer(center_layout);

    siv.add_global_callback(Event::Key(Key::Esc), |s| s.quit());
    siv.add_global_callback('/', |s| {
        s.call_on_name("Input", |view: &mut EditView| view.set_content("/"));
    });

    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let (reader, mut writer) = stream.into_split();
    writer
        .write_all(format!("{}/n", username).as_bytes())
        .await?;

    let wirter = Arc::new(Mutex::new(writer));

    siv.run();

    Ok(())
}

fn send_messages(s: &mut Cursive, text: String) {}
