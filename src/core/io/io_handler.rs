use chrono::Local;
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::broadcast,
};

use crate::core::models::chat_message_model::{ChatMessage, MessageType};

pub async fn handle_connection(
    mut socket: TcpStream,
    tx: broadcast::Sender<String>,
    mut rx: broadcast::Receiver<String>,
) {
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let Some(username) = read_username(&mut reader).await else {
        return;
    };

    broadcast_system_message(&tx, &username, "Joined the chat");
    handle_chat_loop(&mut reader, &mut writer, &username, &tx, &mut rx).await;
    broadcast_system_message(&tx, &username, "Left the chat");

    println!(
        "[{}] {} disconnected",
        Local::now().format("%H:%M:%S"),
        username
    );
}

async fn read_username<R: AsyncRead + Unpin>(reader: &mut BufReader<R>) -> Option<String> {
    let mut username = String::new();
    if reader.read_line(&mut username).await.unwrap_or(0) == 0 {
        return None;
    }
    let username = username.trim().to_string();
    if username.is_empty() {
        return None;
    }
    Some(username)
}

fn broadcast_system_message(tx: &broadcast::Sender<String>, username: &str, content: &str) {
    let message = ChatMessage {
        user_name: username.to_string(),
        message_content: content.to_string(),
        message_type: MessageType::SystemNotification,
        timestamp: Local::now().format("%H:%M:%S").to_string(),
    };
    let _ = tx.send(serde_json::to_string(&message).unwrap());
}

async fn handle_chat_loop<R, W>(
    reader: &mut BufReader<R>,
    writer: &mut W,
    username: &str,
    tx: &broadcast::Sender<String>,
    rx: &mut broadcast::Receiver<String>,
) where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut line = String::new();
    loop {
        tokio::select! {
            result = reader.read_line(&mut line) => {
                match result {
                    Ok(0) => break,
                    Ok(_) => {}
                    Err(_) => break,
                }

                let message = line.trim();
                if message.is_empty() {
                    line.clear();
                    continue;
                }

                broadcast_user_message(tx, username, message);
                line.clear();
            }
            result = rx.recv() => {
                match result {
                    Ok(message) => {
                        if writer.write_all(message.as_bytes()).await.is_err() {
                            break;
                        }
                        if writer.write_all(b"\n").await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }
}

fn broadcast_user_message(tx: &broadcast::Sender<String>, username: &str, content: &str) {
    let message = ChatMessage {
        user_name: username.to_string(),
        message_content: content.to_string(),
        message_type: MessageType::UserMessage,
        timestamp: Local::now().format("%H:%M:%S").to_string(),
    };
    let _ = tx.send(serde_json::to_string(&message).unwrap());
}
