use chrono::Local;
use cursive::Cursive;
use cursive::views::{EditView, TextView};
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
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
        || state.last_typing_sent_ms.compare_exchange(previous, now, Ordering::Relaxed, Ordering::Relaxed).is_err()
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

/// Expands recognized slash emotes anywhere in a message.
fn resolve_inline_tokens(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let (prefix, token) = word
                .char_indices()
                .find(|(_, ch)| *ch == '/')
                .map_or((word, ""), |(i, _)| (&word[..i], &word[i..]));
            let token_end = token
                .char_indices()
                .find(|(_, ch)| !ch.is_ascii_alphanumeric() && *ch != '_')
                .map_or(token.len(), |(i, _)| i);
            if token_end == 0 {
                return word.to_string();
            }
            let command = &token[..token_end];
            let suffix = &token[token_end..];
            resolve_emoticon_command(command)
                .map(|emote| format!("{prefix}{emote}{suffix}"))
                .unwrap_or_else(|| word.to_string())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Resolves slash emote names.
fn resolve_emoticon_command(cmd: &str) -> Option<&'static str> {
    match cmd.to_ascii_lowercase().as_str() {
        "/kiwi" => Some("(◕ө◕) [Kiwi]"),
        "/kawaii" => Some("(✿◕‿◕) [Kawaii]"),
        "/uwu" => Some("(ᵘﻌᵘ) [uwu]"),
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
        "/skull" => Some("[DEAD] (x_x) 💀"),
        "/nerd" => Some("( ⌐■_■) ☝ Actually..."),
        "/clown" => Some("( 🤡 ) [Honk Honk]"),
        "/party" => Some("(★^O^★) ~* Party Time! *~"),
        "/praise" => Some("[PRAISE] \\('o')/ Praise the Sun!"),
        "/copium" => Some("[COPIUM] (´• ω •) *inhales copium*"),
        "/cry" => Some("(╥﹏╥) *sobs*"),
        "/cool" => Some("( •_•)>⌐■-■ (⌐■_■) Deal with it."),
        _ => None,
    }
}

/// Resolves commands that send multi-line ASCII drawings.
fn resolve_ascii_command(cmd: &str) -> Option<&'static str> {
    match cmd.trim().to_ascii_lowercase().as_str() {
        "$cat" => Some(r#" /\_/\
( o.o )
 > ^ <"#),
        "$dog" => Some(r#"/ \__
   (    @\___
   /         O
  /   (_____/
 /_____/   U"#),
        "$bunny" | "$rabbit" => Some(r#"(\_/)
( •.•)
/ >🥕"#),
        "$bear" => Some(r#" ʕ•ᴥ•ʔ
 /|   |\
  |___|"#),
        "$fox" => Some(r#" /\   /\
(  o   o  )
  >  ^  <"#),
        "$panda" => Some(r#"  ʕ●ᴥ●ʔ
 (  ◡  )
  (___)"#),
        "$owl" => Some(r#" ,_,
(O,O)
(   )
-""-"#),
        "$frog" => Some(r#" @..@
(----)
( >__< )"#),
        "$penguin" => Some(r#"  .--.
 (o_o )
 /|_|\
  / \"#),
        "$hamster" => Some(r#" (\__/)
 (='.'=)
 (")_(")"#),
        "$duck" => Some(r#"  __
<(o )___
 ( ._> /
  `---'"#),
        "$koala" => Some(r#" (ᵔᴥᵔ)
 /|   |\
  |___|"#),
        _ => None,
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
