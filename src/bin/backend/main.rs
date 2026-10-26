use chrono::Local;
use std::error::Error;
use tokio::{net::TcpListener, sync::broadcast};
use ttyme::core::constants::config::Constants;
use ttyme::core::io::io_handler::handle_connection;

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
        tokio::spawn(async move { handle_connection(socket, tx, rx).await });
    }
}
