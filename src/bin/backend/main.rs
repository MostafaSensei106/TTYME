use chrono::Local;
use std::{env, error::Error};
use tokio::{net::TcpListener, sync::broadcast};
use ttyme::core::cli::Options;
use ttyme::core::io::io_handler::handle_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = match Options::parse(env::args().skip(1), false) {
        Ok(options) => options,
        Err(error) => {
            eprintln!("{error}\n{}", Options::usage(false));
            std::process::exit(2);
        }
    };
    if options.help {
        println!("{}", Options::usage(false));
        return Ok(());
    }
    let listener = TcpListener::bind(options.address).await?;

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
