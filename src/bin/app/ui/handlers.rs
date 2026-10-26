use std::sync::Arc;
use chrono::Local;
use cursive::views::{EditView, TextView};
use cursive::Cursive;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Mutex;

use crate::ui::message::{format_error_notice, format_info_notice};

/// Holds shared state for the application.
#[derive(Clone)]
pub struct AppState {
    pub writer: Arc<Mutex<OwnedWriteHalf>>,
    pub username: String,
    pub server_addr: String,
}

/// Handles submission from the chat input box.
pub fn handle_submit(s: &mut Cursive, text: String) {
    let text = text.trim().to_string();
    let _ = s.call_on_name("Input", |view: &mut EditView| view.set_content(""));
    let _ = s.focus_name("Input");

    if text.is_empty() {
        return;
    }

    let state = s.user_data::<AppState>().cloned();

    match text.as_str() {
        "/quit" | "/exit" => s.quit(),
        "/clear" => {
            s.call_on_name("Messages", |view: &mut TextView| view.set_content(""));
        }
        "/help" => {
            let help_msg = format_info_notice(
                "===================== COMMAND REFERENCE =====================\n\
                 /help          - Display this command reference\n\
                 /clear         - Clear the chat message history\n\
                 /whoami        - Display your user profile and connection\n\
                 /time          - Display current 12-hour system time\n\
                 /ping          - Ping local connection check\n\
                 /shrug         - Send shrug ascii: ¯\\_(ツ)_/¯\n\
                 /tableflip     - Send tableflip ascii: (╯°□°)╯︵ ┻━┻\n\
                 /unflip        - Send unflip ascii: ┬─┬ノ( º _ ºノ)\n\
                 /me <action>   - Broadcast an action (* User action)\n\
                 /quit, /exit   - Disconnect and exit TTYME\n\
                 =============================================================",
            );
            append_to_messages(s, help_msg);
        }
        "/time" => {
            let now = Local::now();
            let time_notice = format!(
                "Current Local Time: {} ({})",
                now.format("%A, %b %d, %Y - %I:%M:%S %p"),
                now.format("%Z")
            );
            append_to_messages(s, format_info_notice(&time_notice));
        }
        "/whoami" => {
            if let Some(ref st) = state {
                let info = format!(
                    "Username: @{} | Server: {} | Protocol: TCP/JSON | Status: CONNECTED",
                    st.username, st.server_addr
                );
                append_to_messages(s, format_info_notice(&info));
            } else {
                append_to_messages(s, format_info_notice("User state unavailable."));
            }
        }
        "/ping" => {
            append_to_messages(s, format_info_notice("PONG! Connection to server is alive."));
        }
        "/shrug" => {
            send_network_payload(state, "¯\\_(ツ)_/¯".to_string(), s);
        }
        "/tableflip" => {
            send_network_payload(state, "(╯°□°)╯︵ ┻━┻".to_string(), s);
        }
        "/unflip" => {
            send_network_payload(state, "┬─┬ノ( º _ ºノ)".to_string(), s);
        }
        _ if text.starts_with("/me ") => {
            // Forward action message directly to server
            send_network_payload(state, text, s);
        }
        _ if text.starts_with('/') => {
            let error_msg = format_error_notice(
                "Unknown command. Type /help to see all available commands.",
            );
            append_to_messages(s, error_msg);
        }
        _ => {
            send_network_payload(state, text, s);
        }
    }
}

fn send_network_payload(state: Option<AppState>, payload: String, s: &mut Cursive) {
    let Some(st) = state else {
        let err = format_error_notice("Connection is unavailable. Message not sent.");
        append_to_messages(s, err);
        return;
    };

    tokio::spawn(async move {
        let mut writer = st.writer.lock().await;
        let _ = writer.write_all(format!("{payload}\n").as_bytes()).await;
    });
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
