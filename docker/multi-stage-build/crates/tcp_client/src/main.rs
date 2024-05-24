use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = tokio::net::TcpStream::connect("0.0.0.0:8080")
        .await
        .expect("Server not found");
    let (mut rx, mut tx) = stream.split();
    println!("connected to server. enter 'exit' to close connection");
    let mut buf_write = bytes::BytesMut::with_capacity(4096);
    let mut buf_read = bytes::BytesMut::with_capacity(4096);
    loop {
        buf_read.clear();
        buf_write.clear();
        tokio::io::stdin().read_buf(&mut buf_write).await?;
        if buf_write == "exit\n" {
            println!("exiting...");
            break;
        }
        tx.write_all(&buf_write).await?;
        let read_bytes = rx.read_buf(&mut buf_read).await?;
        match read_bytes {
            0 => {
                println!("Connection with server closed");
                break;
            }
            _ => {
                let res = String::from_utf8(buf_read.to_owned().into()).unwrap();
                println!("response message from server: {res}");
            }
        }
    }
    Ok(())
}
