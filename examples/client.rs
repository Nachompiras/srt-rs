use srt_rs::*;
use std::net::SocketAddr;

fn main() {
    let mut args = std::env::args();

    if args.len() < 2 {
        eprintln!(
            "Usage: {} <remote host>:<remote port>",
            args.next().unwrap()
        );
    }

    let _bin = args.next().unwrap();

    let remote = args.next().unwrap();

    let message = "This message should be sent to the other side".as_bytes();

    startup().expect("startup");

    let addr: SocketAddr = remote.parse().expect("Invalid addr:port syntax");

    let ss = SrtSocket::new().expect("create_socket");

    ss.connect(addr).expect("connect");

    for i in 0..100 {
        println!("Sending message {}", i);
        ss.send(message).expect("send");

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    ss.close().expect("close");

    cleanup().expect("cleanup");
}