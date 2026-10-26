use std::sync::Arc;
use cursive::views::{EditView, TextView};
use cursive::Cursive;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Mutex;

use crate::ui::message::{format_error_notice, format_info_notice};

/// Handles submission from the chat input box.
pub fn handle_submit(s: &mut Cursive, text: String) {
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
        "/help" => {
            let help_msg = format_info_notice(
                "Available Commands:\n  /help   - Show this help message\n  /clear  - Clear chat history\n  /quit   - Exit TTYME",
            );
            append_to_messages(s, help_msg);
        }
        _ if text.starts_with('/') => {
            let error_msg = format_error_notice(
                "Unknown command. Type /help to see all available commands.",
            );
            append_to_messages(s, error_msg);
        }
        _ => {
            let Some(writer) = s.user_data::<Arc<Mutex<OwnedWriteHalf>>>().cloned() else {
                let err = format_error_notice("Connection is unavailable. Message not sent.");
                append_to_messages(s, err);
                return;
            };

            tokio::spawn(async move {
                let mut writer = writer.lock().await;
                let _ = writer.write_all(format!("{text}\n").as_bytes()).await;
            });
        }
    }
}

/// Appends any StyledString to the chat messages panel.
pub fn append_to_messages<T>(s: &mut Cursive, content: T)
where
    T: Into<cursive::utils::markup::StyledString> + 'static,
{
    s.call_on_name("Messages", |view: &mut TextView| {
        view.append(content);
    });
}
