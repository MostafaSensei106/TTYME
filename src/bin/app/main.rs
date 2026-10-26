use cursive::CursiveExt;
use std::{env, error::Error, sync::Arc};
use ttyme::core::constants::config::Constants;

mod network;
mod ui;

use network::client::{connect_server, spawn_reader};
use ui::layout::build_ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let username = match env::args().nth(1) {
        Some(name) if !name.trim().is_empty() => name.trim().to_string(),
        _ => {
            eprintln!("Usage: cargo run --bin app -- <username>");
            std::process::exit(1);
        }
    };

    let config = Constants::default();
    let server_addr = format!("{}:{}", config.host, config.port);

    // 1. Establish connection and complete initial handshake
    let (reader, writer) = connect_server(config.host, config.port, &username).await?;

    // 2. Construct TUI view hierarchy
    let mut siv = build_ui(&username, &server_addr);
    let sink = siv.cb_sink().clone();

    // 3. Store writer handle in Cursive user_data for UI callbacks
    siv.set_user_data(Arc::clone(&writer));

    // 4. Start background listener for incoming broadcast messages
    spawn_reader(reader, sink, username);

    // 5. Run the Cursive interactive event loop
    siv.run();

    Ok(())
}
