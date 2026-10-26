use std::error::Error;
use std::sync::Arc;
use cursive::{CbSink, Cursive};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use ttyme::core::models::chat_message_model::ChatMessage;

use crate::ui::handlers::append_to_messages;
use crate::ui::message::{format_chat_message, format_error_notice};

/// Establishes the TCP connection and completes the initial username handshake.
pub async fn connect_server(
    host: &str,
    port: u16,
    username: &str,
) -> Result<(OwnedReadHalf, Arc<Mutex<OwnedWriteHalf>>), Box<dyn Error>> {
    let stream = TcpStream::connect(format!("{host}:{port}")).await?;
    let (reader, mut writer) = stream.into_split();

    // Handshake: send username with trailing newline
    writer.write_all(format!("{username}\n").as_bytes()).await?;

    let writer = Arc::new(Mutex::new(writer));
    Ok((reader, writer))
}

/// Spawns an asynchronous background task to read incoming messages and update the UI.
pub fn spawn_reader(reader: OwnedReadHalf, sink: CbSink, username: String) {
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

            let formatted = format_chat_message(&message, &username);
            let _ = sink.send(Box::new(move |siv: &mut Cursive| {
                append_to_messages(siv, formatted);
            }));
        }
    });
}
