use chrono::Local;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
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
    let mut username = String::new();

    if reader.read_line(&mut username).await.unwrap_or(0) == 0 {
        return;
    }
    let username = username.trim().to_string();

    let join_message = serde_json::to_string(&ChatMessage {
        user_name: username.clone(),
        message_content: "Joined the chat".to_string(),
        message_type: MessageType::SystemNotification,
        timestamp: Local::now().format("%H:%M:%S").to_string(),
    })
    .unwrap();

    let _ = tx.send(join_message);

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

                let chat_message = ChatMessage {
                    user_name: username.clone(),
                    message_content: message.to_string(),
                    message_type: MessageType::UserMessage,
                    timestamp: Local::now().format("%H:%M:%S").to_string(),
                };

                let json = serde_json::to_string(&chat_message).unwrap();
                tx.send(json).unwrap();
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

    let leave_message = serde_json::to_string(&ChatMessage {
        user_name: username.clone(),
        message_content: "Left the chat".to_string(),
        message_type: MessageType::SystemNotification,
        timestamp: Local::now().format("%H:%M:%S").to_string(),
    })
    .unwrap();
    tx.send(leave_message).unwrap();

    println!(
        "[{}] {} disconnected",
        Local::now().format("%H:%M:%S"),
        username
    );
}
