use srt_rs::{self as srt};
use std::net::SocketAddr;
use tokio::io::Result;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    srt::startup()?;

    let listen_addr = "127.0.0.1:1234";
    let output_addr: SocketAddr = "127.0.0.1:9090".parse().unwrap();

    let listener = srt::builder()
        .set_live_transmission_type()
        .set_passphrase(Some(String::from("oli123")))
        .listen(listen_addr, 2)?;

    loop {
        let (conn, peer) = match listener.accept() {
            Ok((conn, peer)) => (conn, peer),
            Err(e) => {
                eprintln!("accept failed: {e}");
                continue;
            }
        };
        println!("Accepted connection from {peer}");

        let udp = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;
        tokio::spawn(async move {
            if let Err(e) = pump(conn.socket, udp, output_addr).await {
                eprintln!("connection {peer} failed: {e}");
            }
        });
    }
}

async fn pump(
    srt_conn: srt::SrtSocket,
    udp: tokio::net::UdpSocket,
    target: SocketAddr,
) -> Result<()> {
    let mut buf = [0u8; 2048];
    loop {
        let n = srt_conn.recv(&mut buf)?;
        if n == 0 {
            break;
        }
        udp.send_to(&buf[..n], target).await?;
    }
    Ok(())
}
