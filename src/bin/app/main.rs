use cursive::CursiveExt;
use std::{env, error::Error};
use ttyme::core::cli::Options;

mod network;
mod ui;

use network::client::{connect_server, spawn_reader};
use ui::handlers::AppState;
use ui::layout::build_ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = match Options::parse(env::args().skip(1), true) {
        Ok(options) => options,
        Err(error) => {
            eprintln!("{error}\n{}", Options::usage(true));
            std::process::exit(2);
        }
    };
    if options.help {
        println!("{}", Options::usage(true));
        return Ok(());
    }
    let username = options.username.unwrap();

    let server_addr = options.address.to_string();

    // Establish connection and complete initial handshake
    let (reader, writer) = connect_server(options.address, &username).await?;

    //  Construct TUI view hierarchy
    let mut siv = build_ui(&username, &server_addr);
    let sink = siv.cb_sink().clone();

    // tore application state in Cursive user_data for UI callbacks
    siv.set_user_data(AppState {
        writer,
        username: username.clone(),
        server_addr: server_addr.clone(),
        last_typing_sent_ms: std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0)),
    });

    //  Start background listener for incoming broadcast messages
    spawn_reader(reader, sink.clone(), username);

    // tart live ticking clock updater (12-hour AM/PM format)
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

    // Run the Cursive interactive event loop
    siv.run();

    Ok(())
}
