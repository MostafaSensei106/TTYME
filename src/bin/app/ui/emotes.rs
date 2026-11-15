/// Expands recognized slash emotes anywhere in a message.
pub fn resolve_inline_tokens(text: &str) -> String {
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
pub fn resolve_emoticon_command(cmd: &str) -> Option<&'static str> {
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
