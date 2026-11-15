use chrono::Local;
use clap::Parser;
use std::error::Error;
use tokio::{net::TcpListener, sync::broadcast};
use ttyme::core::cli::BackendOptions;
use ttyme::core::io::io_handler::handle_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = BackendOptions::parse();
    let listener = TcpListener::bind(options.address()).await?;

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
            Local::now().format("%I:%M:%S %p"),
            addr
        );
        let tx = tx.clone();
        let rx = tx.subscribe();
        tokio::spawn(async move { handle_connection(socket, tx, rx).await });
    }
}
