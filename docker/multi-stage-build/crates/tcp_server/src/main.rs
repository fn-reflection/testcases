use std::sync::Arc;

use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = Arc::new(tokio::net::TcpListener::bind("0.0.0.0:8080").await?);
    loop {
        let listener_arc = listener.clone();
        let (mut socket, addr) = listener_arc.accept().await.unwrap();
        tokio::spawn(async move {
            println!("socket: {socket:?}, addr: {addr}");
            let mut buf = bytes::BytesMut::with_capacity(4096);
            loop {
                buf.clear();
                let read_bytes = socket.read_buf(&mut buf).await.unwrap();
                match read_bytes {
                    0 => {
                        println!("Connection with {addr} closed");
                        break;
                    }
                    _ => {
                        socket.write_all(&buf).await.unwrap();
                        let msg = String::from_utf8(buf.to_owned().into())
                            .expect("failed to convert str");
                        println!("request message from {addr}: {msg}");
                    }
                }
            }
        });
    }
}
