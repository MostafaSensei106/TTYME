use std::error::Error;
mod core;
use chrono::Local;
use core::constants::config::Constants;
use tokio::{net::TcpListener, sync::broadcast};

use crate::core::io::io::handle_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(format!(
        "{}:{}",
        Constants::default().host,
        Constants::default().port
    ))
    .await?;

    println!(
        r#"
╔══════════════════════════════════════════╗
║                                          ║
║  ████████╗████████╗██╗   ██╗███╗   ███╗  ║
║  ╚══██╔══╝╚══██╔══╝╚██╗ ██╔╝████╗ ████║  ║
║     ██║      ██║    ╚████╔╝ ██╔████╔██║  ║
║     ██║      ██║     ╚██╔╝  ██║╚██╔╝██║  ║
║     ██║      ██║      ██║   ██║ ╚═╝ ██║  ║
║     ╚═╝      ╚═╝      ╚═╝   ╚═╝     ╚═╝  ║
║                                          ║
╚══════════════════════════════════════════╝

    TTYME server started successfully
    Listening on: {host}:{port}

    Press Ctrl+C to stop the server.
"#,
        host = listener.local_addr()?.ip(),
        port = listener.local_addr()?.port(),
    );

    let (tx, _) = broadcast::channel::<String>(50);
    loop {
        let (socket, addr) = listener.accept().await?;
        println!(
            "\n┌─[{}] New Connection\n└─ Address: {}\n",
            Local::now().format("%H:%M:%S"),
            addr
        );
        let tx = tx.clone();
        let rx = tx.subscribe();
        tokio::spawn(async move { handle_connection(socket, tx, rx) });
    }

    Ok(())
}
