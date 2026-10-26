use std::{env, error::Error, sync::Arc};

use chrono::Local;
use cursive::{
    Cursive, CursiveExt,
    align::HAlign,
    event::{Event, Key},
    view::{Nameable, Resizable, Scrollable},
    views::{Dialog, DummyView, EditView, LinearLayout, Panel, ScrollView, TextView},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpStream, tcp::OwnedWriteHalf},
    sync::Mutex,
};

use crate::core::theme::retro::create_retro_theme;
use ttyme::core::models::chat_message_model::{ChatMessage, MessageType};
mod core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let username = env::args()
        .nth(1)
        .expect("Please provide a username as an argument");

    let mut siv = create_ui(&username);

    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let (reader, mut writer) = stream.into_split();
    writer.write_all(format!("{username}\n").as_bytes()).await?;

    let writer = Arc::new(Mutex::new(writer));
    siv.set_user_data(Arc::clone(&writer));
    spawn_reader(reader, siv.cb_sink().clone());

    siv.run();
    Ok(())
}

fn create_ui(username: &str) -> Cursive {
    let mut siv = Cursive::new();
    siv.set_theme(create_retro_theme());

    let header = TextView::new(format!(
        "• {username} • {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    ))
    .h_align(HAlign::Center);
    let messages = ScrollView::new(
        TextView::new("")
            .with_name("Messages")
            .min_height(20)
            .scrollable(),
    )
    .scroll_strategy(cursive::view::ScrollStrategy::StickToBottom)
    .min_height(60)
    .full_width();
    let input = EditView::new()
        .on_submit(|s, text| send_messages(s, text.to_string()))
        .with_name("Input")
        .max_height(3)
        .min_width(50)
        .full_width();
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
        .child(Panel::new(TextView::new(
            "ESC: quit | Enter: send | Commands: /help, /clear, /quit",
        )));

    siv.add_fullscreen_layer(
        LinearLayout::horizontal()
            .child(DummyView.full_width())
            .child(layout)
            .child(DummyView.full_width()),
    );
    siv.add_global_callback(Event::Key(Key::Esc), |s| s.quit());
    siv.add_global_callback('/', |s| {
        s.call_on_name("Input", |view: &mut EditView| view.set_content("/"));
    });
    siv
}

fn spawn_reader(reader: tokio::net::tcp::OwnedReadHalf, sink: cursive::CbSink) {
    tokio::spawn(async move {
        let mut lines = BufReader::new(reader).lines();
        loop {
            let line = match lines.next_line().await {
                Ok(Some(line)) => line,
                _ => break,
            };
            let Ok(message) = serde_json::from_str::<ChatMessage>(&line) else {
                continue;
            };
            let formatted = format_message(message);
            let _ = sink.send(Box::new(move |siv: &mut Cursive| {
                siv.call_on_name("Messages", |view: &mut TextView| {
                    view.append(format!("{formatted}\n"));
                });
            }));
        }
    });
}

fn format_message(message: ChatMessage) -> String {
    match message.message_type {
        MessageType::UserMessage => format!(
            "-[{}]\n -> {} > {}",
            message.timestamp, message.user_name, message.message_content
        ),
        MessageType::SystemNotification => {
            format!(
                "[{}] [System] {}",
                message.timestamp, message.message_content
            )
        }
    }
}

fn send_messages(s: &mut Cursive, text: String) {
    let text = text.trim().to_string();
    s.call_on_name("Input", |view: &mut EditView| view.set_content(""));
    if text.is_empty() {
        return;
    }

    match text.as_str() {
        "/quit" => s.quit(),
        "/clear" => {
            s.call_on_name("Messages", |view: &mut TextView| view.set_content(""));
        }
        "/help" => append_local_message(
            s,
            "Commands: /help, /clear, /quit. Enter a message to send it.".to_string(),
        ),
        _ if text.starts_with('/') => {
            append_local_message(s, "Unknown command. Use /help.".to_string())
        }
        _ => {
            let Some(writer) = s.user_data::<Arc<Mutex<OwnedWriteHalf>>>().cloned() else {
                append_local_message(s, "Connection is unavailable.".to_string());
                return;
            };
            tokio::spawn(async move {
                let mut writer = writer.lock().await;
                let _ = writer.write_all(format!("{text}\n").as_bytes()).await;
            });
        }
    }
}

fn append_local_message(s: &mut Cursive, message: String) {
    s.call_on_name("Messages", |view: &mut TextView| {
        view.append(format!("{message}\n"));
    });
}
