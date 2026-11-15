use cursive::{CbSink, Cursive};
use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::Mutex;
use ttyme::core::models::chat_message_model::ChatMessage;

use crate::ui::handlers::append_to_messages;
use crate::ui::message::{format_chat_message, format_error_notice};

/// Establishes the TCP connection and completes the initial username handshake.
pub async fn connect_server(
    address: SocketAddr,
    username: &str,
) -> Result<(OwnedReadHalf, Arc<Mutex<OwnedWriteHalf>>), Box<dyn Error>> {
    let stream = TcpStream::connect(address).await?;
    let (reader, mut writer) = stream.into_split();

    // Handshake: send username with trailing newline
    writer.write_all(format!("{username}\n").as_bytes()).await?;

    let writer = Arc::new(Mutex::new(writer));
    Ok((reader, writer))
}

/// Spawns an asynchronous background task to read incoming messages and update the UI.
pub fn spawn_reader(reader: OwnedReadHalf, sink: CbSink, username: String) {
    let typing_generation = Arc::new(AtomicU64::new(0));
    tokio::spawn(async move {
        let mut lines = BufReader::new(reader).lines();
        loop {
            let line = match lines.next_line().await {
                Ok(Some(line)) => line,
                _ => {
                    // Connection closed or broken
                    let disconnect_notice =
                        format_error_notice("Disconnected from server. Chat is offline.");
                    let _ = sink.send(Box::new(move |siv: &mut Cursive| {
                        append_to_messages(siv, disconnect_notice);
                    }));
                    break;
                }
            };

            let Ok(message) = serde_json::from_str::<ChatMessage>(&line) else {
                continue;
            };

            match &message.message_type {
                ttyme::core::models::chat_message_model::MessageType::Typing => {
                    if message.user_name.eq_ignore_ascii_case(&username) {
                        continue;
                    }
                    let generation = typing_generation.fetch_add(1, Ordering::Relaxed) + 1;
                    let name = message.user_name.clone();
                    let _ = sink.send(Box::new(move |siv: &mut Cursive| {
                        siv.call_on_name("Typing", |view: &mut cursive::views::TextView| {
                            view.set_content(format!("{name} is typing…"));
                        });
                    }));
                    let expiry_sink = sink.clone();
                    let expiry_generation = typing_generation.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                        let _ = expiry_sink.send(Box::new(move |siv: &mut Cursive| {
                            if expiry_generation.load(Ordering::Relaxed) == generation {
                                siv.call_on_name("Typing", |view: &mut cursive::views::TextView| view.set_content(""));
                            }
                        }));
                    });
                }
                ttyme::core::models::chat_message_model::MessageType::UserMessage => {
                    typing_generation.fetch_add(1, Ordering::Relaxed);
                    let formatted = format_chat_message(&message, &username);
                    let _ = sink.send(Box::new(move |siv: &mut Cursive| {
                        siv.call_on_name("Typing", |view: &mut cursive::views::TextView| view.set_content(""));
                        append_to_messages(siv, formatted);
                    }));
                }
                ttyme::core::models::chat_message_model::MessageType::SystemNotification => {
                    let formatted = format_chat_message(&message, &username);
                    let _ = sink.send(Box::new(move |siv: &mut Cursive| {
                        append_to_messages(siv, formatted);
                    }));
                }
            }
        }
    });
}
