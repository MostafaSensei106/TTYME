use chrono::Local;
use cursive::theme::{BaseColor, Color, Effect, Style};
use cursive::utils::markup::StyledString;
use ttyme::core::models::chat_message_model::{ChatMessage, MessageType};

/// Formats an incoming broadcast message into a clean, ASCII-styled terminal line.
pub fn format_chat_message(message: &ChatMessage, current_user: &str) -> StyledString {
    let mut s = StyledString::new();
    let time_style = Style::from(Color::Dark(BaseColor::Blue));

    match message.message_type {
        MessageType::UserMessage => {
            let is_me = message.user_name.trim().eq_ignore_ascii_case(current_user.trim());

            // 1. Timestamp in 12-hour format: [06:30:15 PM]
            s.append_styled(format!("[{}] ", message.timestamp), time_style);

            // Check if this is an IRC-style action (/me)
            if let Some(action) = message.message_content.strip_prefix("/me ") {
                let action_style = Style::from(Color::Light(BaseColor::Magenta)).combine(Effect::Bold);
                s.append_styled(format!("* {} ", message.user_name), action_style);
                s.append_styled(action, Style::from(Color::Light(BaseColor::Magenta)));
                s.append_plain("\n");
                return s;
            }

            // 2. Sender Badge
            if is_me {
                let me_style = Style::from(Color::Light(BaseColor::Green)).combine(Effect::Bold);
                s.append_styled("<You>", me_style);
                s.append_styled(" >> ", Style::from(Color::Light(BaseColor::Green)));
            } else {
                let user_style = Style::from(Color::Light(BaseColor::Cyan)).combine(Effect::Bold);
                s.append_styled(format!("<@{}>", message.user_name), user_style);
                s.append_styled(" >> ", Style::from(Color::Light(BaseColor::Cyan)));
            }

            // 3. Message Body
            s.append_plain(&message.message_content);
            s.append_plain("\n");
        }
        MessageType::SystemNotification => {
            s.append_styled(format!("[{}] ", message.timestamp), time_style);
            s.append_styled(
                "*** [SYSTEM] ",
                Style::from(Color::Light(BaseColor::Yellow)).combine(Effect::Bold),
            );
            s.append_styled(
                &message.message_content,
                Style::from(Color::Light(BaseColor::Yellow)),
            );
            s.append_plain("\n");
        }
    }
    s
}

/// Formats an informational notice (e.g. /help or /time).
pub fn format_info_notice(notice: &str) -> StyledString {
    let mut s = StyledString::new();
    let time = Local::now().format("%I:%M:%S %p").to_string();
    let time_style = Style::from(Color::Dark(BaseColor::Blue));

    s.append_styled(format!("[{time}] "), time_style);
    s.append_styled(
        "*** [INFO] ",
        Style::from(Color::Light(BaseColor::Cyan)).combine(Effect::Bold),
    );
    s.append_styled(notice, Style::from(Color::Light(BaseColor::White)));
    s.append_plain("\n");
    s
}

/// Formats an error or warning notice.
pub fn format_error_notice(notice: &str) -> StyledString {
    let mut s = StyledString::new();
    let time = Local::now().format("%I:%M:%S %p").to_string();
    let time_style = Style::from(Color::Dark(BaseColor::Blue));

    s.append_styled(format!("[{time}] "), time_style);
    s.append_styled(
        "[!] [ERROR] ",
        Style::from(Color::Light(BaseColor::Red)).combine(Effect::Bold),
    );
    s.append_styled(notice, Style::from(Color::Light(BaseColor::Red)));
    s.append_plain("\n");
    s
}
