use chrono::Local;
use cursive::theme::{BaseColor, Color, Effect, Style};
use cursive::utils::markup::StyledString;
use ttyme::core::models::chat_message_model::{ChatMessage, MessageType};

/// Formats an incoming broadcast message into a colored, structured StyledString.
pub fn format_chat_message(message: &ChatMessage, current_user: &str) -> StyledString {
    let mut s = StyledString::new();
    let time_style = Style::from(Color::Dark(BaseColor::Blue));

    match message.message_type {
        MessageType::UserMessage => {
            let is_me = message
                .user_name
                .trim()
                .eq_ignore_ascii_case(current_user.trim());

            // Timestamp: [HH:MM:SS]
            s.append_styled(format!("[{}] ", message.timestamp), time_style);

            if is_me {
                // You >
                let me_style = Style::from(Color::Light(BaseColor::Green)).combine(Effect::Bold);
                s.append_styled("You", me_style);
                s.append_styled(" ❯ ", Style::from(Color::Light(BaseColor::Green)));
            } else {
                // @username >
                let user_style = Style::from(Color::Light(BaseColor::Cyan)).combine(Effect::Bold);
                s.append_styled(format!("@{}", message.user_name), user_style);
                s.append_styled(" ❯ ", Style::from(Color::Light(BaseColor::Cyan)));
            }

            // Message text
            s.append_plain(&message.message_content);
            s.append_plain("\n");
        }
        MessageType::SystemNotification => {
            s.append_styled(format!("[{}] ", message.timestamp), time_style);
            s.append_styled(
                "⚡ [System] ",
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

/// Formats a local informational notice (e.g. commands help).
pub fn format_info_notice(notice: &str) -> StyledString {
    let mut s = StyledString::new();
    let time = Local::now().format("%H:%M:%S").to_string();
    let time_style = Style::from(Color::Dark(BaseColor::Blue));

    s.append_styled(format!("[{time}] "), time_style);
    s.append_styled(
        "ℹ [Info] ",
        Style::from(Color::Light(BaseColor::Magenta)).combine(Effect::Bold),
    );
    s.append_styled(notice, Style::from(Color::Light(BaseColor::Magenta)));
    s.append_plain("\n");
    s
}

/// Formats a local warning or error notice.
pub fn format_error_notice(notice: &str) -> StyledString {
    let mut s = StyledString::new();
    let time = Local::now().format("%H:%M:%S").to_string();
    let time_style = Style::from(Color::Dark(BaseColor::Blue));

    s.append_styled(format!("[{time}] "), time_style);
    s.append_styled(
        "✖ [Error] ",
        Style::from(Color::Light(BaseColor::Red)).combine(Effect::Bold),
    );
    s.append_styled(notice, Style::from(Color::Light(BaseColor::Red)));

    s.append_plain("\n");
    s
}
