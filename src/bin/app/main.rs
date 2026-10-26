use cursive::CursiveExt;
use std::{env, error::Error};
use ttyme::core::constants::config::Constants;

mod network;
mod ui;

use network::client::{connect_server, spawn_reader};
use ui::handlers::AppState;
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

    // 3. Store application state in Cursive user_data for UI callbacks
    siv.set_user_data(AppState {
        writer,
        username: username.clone(),
        server_addr: server_addr.clone(),
    });

    // 4. Start background listener for incoming broadcast messages
    spawn_reader(reader, sink.clone(), username);

    // 5. Start live ticking clock updater (12-hour AM/PM format)
    let clock_sink = sink;
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(tokio::time::Duration::from_secs(1));
        loop {
            ticker.tick().await;
            let time_str = chrono::Local::now().format("%I:%M:%S %p").to_string();
            let res = clock_sink.send(Box::new(move |siv: &mut cursive::Cursive| {
                siv.call_on_name("Clock", |view: &mut cursive::views::TextView| {
                    view.set_content(format!("[ {time_str} ]"));
                });
            }));
            if res.is_err() {
                break;
            }
        }
    });

    // 6. Run the Cursive interactive event loop
    siv.run();

    Ok(())
}
