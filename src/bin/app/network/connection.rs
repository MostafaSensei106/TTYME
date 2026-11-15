use std::{error::Error, net::SocketAddr, sync::Arc};

use tokio::{
    io::AsyncWriteExt,
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::Mutex,
};

/// Opens the server connection and sends the username handshake.
pub async fn connect_server(
    address: SocketAddr,
    username: &str,
) -> Result<(OwnedReadHalf, Arc<Mutex<OwnedWriteHalf>>), Box<dyn Error>> {
    let stream = TcpStream::connect(address).await?;
    let (reader, mut writer) = stream.into_split();
    writer.write_all(format!("{username}\n").as_bytes()).await?;
    Ok((reader, Arc::new(Mutex::new(writer))))
}
