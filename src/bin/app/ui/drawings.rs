/// Resolves commands that send multi-line ASCII drawings.
pub fn resolve_ascii_command(cmd: &str) -> Option<&'static str> {
    match cmd.trim().to_ascii_lowercase().as_str() {
        "$cat" => Some(
            r#" /\_/\
( o.o )
 > ^ <"#,
        ),
        "$dog" => Some(
            r#"/ \__
   (    @\___
   /         O
  /   (_____/
 /_____/   U"#,
        ),
        "$bunny" | "$rabbit" => Some(
            r#"(\_/)
( •.•)
/ >🥕"#,
        ),
        "$bear" => Some(
            r#" ʕ•ᴥ•ʔ
 /|   |\
  |___|"#,
        ),
        "$fox" => Some(
            r#" /\   /\
(  o   o  )
  >  ^  <"#,
        ),
        "$panda" => Some(
            r#"  ʕ●ᴥ●ʔ
 (  ◡  )
  (___)"#,
        ),
        "$owl" => Some(
            r#" ,_,
(O,O)
(   )
-""-"#,
        ),
        "$frog" => Some(
            r#" @..@
(----)
( >__< )"#,
        ),
        "$penguin" => Some(
            r#"  .--.
 (o_o )
 /|_|\
  / \"#,
        ),
        "$hamster" => Some(
            r#" (\__/)
 (='.'=)
 (")_(")"#,
        ),
        "$duck" => Some(
            r#"  __
<(o )___
 ( ._> /
  `---'"#,
        ),
        "$koala" => Some(
            r#" (ᵔᴥᵔ)
 /|   |\
  |___|"#,
        ),
        _ => None,
    }
}
