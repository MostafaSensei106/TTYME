use chrono::Local;
use cursive::Cursive;
use cursive::views::{EditView, TextView};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Mutex;

use crate::ui::{
    drawings::resolve_ascii_command,
    emotes::resolve_inline_tokens,
    message::{format_error_notice, format_info_notice},
};

/// Holds shared state for the application.
#[derive(Clone)]
pub struct AppState {
    pub writer: Arc<Mutex<OwnedWriteHalf>>,
    pub username: String,
    pub server_addr: String,
    pub last_typing_sent_ms: Arc<AtomicU64>,
}

/// Sends a throttled typing event while the user edits a non-empty message.
pub fn handle_input_change(s: &mut Cursive, content: &str) {
    if content.trim().is_empty() {
        return;
    }
    let Some(state) = s.user_data::<AppState>().cloned() else {
        return;
    };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    let previous = state.last_typing_sent_ms.load(Ordering::Relaxed);
    if now.saturating_sub(previous) < 900
        || state
            .last_typing_sent_ms
            .compare_exchange(previous, now, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
    {
        return;
    }
    tokio::spawn(async move {
        let mut writer = state.writer.lock().await;
        let _ = writer.write_all(b"\x01typing\n").await;
    });
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

    // Expand known emotes and ASCII drawings anywhere in the message.
    let text = resolve_inline_tokens(&text);
    if let Some(drawing) = resolve_ascii_command(&text) {
        send_network_payload(state, drawing.to_string(), s);
        return;
    }

    match text.as_str() {
        "/quit" | "/exit" => s.quit(),
        "/clear" => {
            s.call_on_name("Messages", |view: &mut TextView| view.set_content(""));
        }
        "/help" => {
            let help_msg = format_info_notice(
                "===================== COMMAND REFERENCE =====================\n\
                 [ Core Commands ]\n\
                   /help        - Display this command reference\n\
                   /clear       - Clear the chat message history\n\
                   /whoami      - Display your user profile and connection\n\
                   /time        - Display current 12-hour system time\n\
                   /ping        - Ping local connection check\n\
                   /me <act>    - Broadcast an action (* User action)\n\
                   /quit, /exit - Disconnect and exit TTYME\n\
                 \n\
                 [ Emotes & Memes ]\n\
                   /kiwi        - (◕ө◕) [Kiwi]\n\
                   /kawaii      - (✿◕‿◕) [Kawaii]\n\
                   /uwu         - (ᵘﻌᵘ) [uwu]\n\
                   /shrug       - ¯\\_(ツ)_/¯\n\
                   /tableflip   - (╯°□°)╯︵ ┻━┻\n\
                   /unflip      - ┬─┬ノ( º _ ºノ)\n\
                   /gigachad    - (•_•) ( •_•)>⌐■-■ (⌐■_■) [CHAD]\n\
                   /bruh        - (ಠ_ಠ) Bruh...\n\
                   /rip         - [RIP] Press F to pay respects\n\
                   /gg          - [GG] Good game, well played!\n\
                   /ez          - [EZ] Easy peasy, lemon squeezy!\n\
                   /noob        - [NOOB] Skill issue detected!\n\
                   /sus         - ඞ [SUS] Impostor detected!\n\
                   /facepalm    - (－‸ლ) *facepalm*\n\
                   /cheers      - ( ﾟヮﾟ)/* [Cheers!]\n\
                   /hype        - \\(^o^)/ HYPE HYPE HYPE!\n\
                   /doge        - (◕ᴥ◕) Such chat, much wow!\n\
                   /cat         - (=^･ω･^=) Meow!\n\
                   /magic       - (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧ Magic!\n\
                   /rage        - (ノಠ益ಠ)ノ彡┻━┻\n\
                   /dance       - ♪┏(・o･)┛♪┗ ( ･o･) ┓♪\n\
                   /flex        - ᕙ(⇀‸↼‶)ᕗ FLEX!\n\
                   /sleep       - (ᴗ_ ᴗ。) zzz...\n\
                   /salute      - (｀-´)ゞ o7\n\
                   /popcorn     - ( ˘▽˘)っ♨ [Eats popcorn]\n\
                   /skull       - [DEAD] (x_x)\n\
                   /nerd        - ( ⌐■_■) ☝ Actually...\n\
                   /clown       - ( 🤡 ) [Honk Honk]\n\
                   /party       - (★^O^★) ~* Party Time! *~\n\
                   /praise      - [PRAISE] \\('o')/ Praise the Sun!\n\
                   /copium      - [COPIUM] (´• ω •) *inhales copium*\n\
                   /cry         - (╥﹏╥) *sobs*\n\
                   /cool        - ( •_•)>⌐■-■ (⌐■_■) Deal with it.\n\
                 \n\
                 [ ASCII Art ]\n\
                   $cat         - Send a cat drawing\n\
                   $dog         - Send a dog drawing\n\
                   $bunny, $rabbit - Send a bunny drawing\n\
                   $bear        - Send a bear drawing\n\
                   $fox         - Send a fox drawing\n\
                   $panda       - Send a panda drawing\n\
                   $owl         - Send an owl drawing\n\
                   $frog        - Send a frog drawing\n\
                   $penguin     - Send a penguin drawing\n\
                   $hamster     - Send a hamster drawing\n\
                   $duck        - Send a duck drawing\n\
                   $koala       - Send a koala drawing\n\
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
            append_to_messages(
                s,
                format_info_notice("PONG! Connection to server is alive."),
            );
        }
        _ if text.starts_with("/me ") => {
            send_network_payload(state, text, s);
        }
        _ if text.starts_with('/') => {
            let error_msg =
                format_error_notice("Unknown command. Type /help to see all available commands.");
            append_to_messages(s, error_msg);
        }
        _ => {
            send_network_payload(state, text, s);
        }
    }
}

/// Sends a payload to the server. Multi-line payloads are sent one line at a
/// time, since the server splits messages on '\n'.
fn send_network_payload(state: Option<AppState>, payload: String, s: &mut Cursive) {
    let Some(st) = state else {
        let err = format_error_notice("Connection is unavailable. Message not sent.");
        append_to_messages(s, err);
        return;
    };

    tokio::spawn(async move {
        // Hold the lock for the whole loop so other messages can't
        // interleave between the lines of a drawing.
        let mut writer = st.writer.lock().await;
        for line in payload.lines() {
            let _ = writer.write_all(format!("{line}\n").as_bytes()).await;
        }
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
