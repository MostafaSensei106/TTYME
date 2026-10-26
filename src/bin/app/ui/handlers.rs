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

    // Check if the command is a quick meme / emoticon command
    if let Some(emoticon) = resolve_emoticon_command(&text) {
        send_network_payload(state, emoticon.to_string(), s);
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
        _ if text.starts_with("/me ") => {
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

/// Resolves quick ASCII emotes and memes commands.
fn resolve_emoticon_command(cmd: &str) -> Option<&'static str> {
    match cmd {
        "/eggplant" => Some("(づ｡◕‿‿◕｡)づ [Eggplant] ~-(:::)"),
        "/cucumber" => Some("[Cucumber] ╰( ͡° ͜ʖ ͡° )つ──☆*:・ﾟ"),
        "/pickle" => Some("[Pickle] ( •_•) Pickle Rick!"),
        "/shrug" => Some("¯\\_(ツ)_/¯"),
        "/tableflip" => Some("(╯°□°)╯︵ ┻━┻"),
        "/unflip" => Some("┬─┬ノ( º _ ºノ)"),
        "/gigachad" | "/chad" => Some("(•_•) ( •_•)>⌐■-■ (⌐■_■) [CHAD]"),
        "/bruh" => Some("(ಠ_ಠ) Bruh..."),
        "/rip" | "/f" => Some("[RIP] Press F to pay respects"),
        "/gg" => Some("[GG] Good game, well played!"),
        "/ez" => Some("[EZ] Easy peasy, lemon squeezy!"),
        "/noob" => Some("[NOOB] Skill issue detected!"),
        "/sus" => Some("ඞ [SUS] Impostor detected!"),
        "/facepalm" => Some("(－‸ლ) *facepalm*"),
        "/cheers" => Some("( ﾟヮﾟ)/* [Cheers!]"),
        "/hype" => Some("\\(^o^)/ HYPE HYPE HYPE!"),
        "/doge" => Some("(◕ᴥ◕) Such chat, much wow!"),
        "/cat" => Some("(=^･ω･^=) Meow!"),
        "/magic" => Some("(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧ Magic!"),
        "/rage" => Some("(ノಠ益ಠ)ノ彡┻━┻"),
        "/dance" => Some("♪┏(・o･)┛♪┗ ( ･o･) ┓♪"),
        "/flex" => Some("ᕙ(⇀‸↼‶)ᕗ FLEX!"),
        "/sleep" => Some("(ᴗ_ ᴗ。) zzz..."),
        "/salute" | "/o7" => Some("(｀-´)ゞ o7"),
        "/popcorn" => Some("( ˘▽˘)っ♨ [Eats popcorn]"),
        _ => None,
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
