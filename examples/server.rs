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

    let addr: SocketAddr = remote.parse().expect("Invalid addr:port syntax");

    startup().expect("startup");

    let ss = SrtSocket::new().expect("create_socket");

    ss.bind(addr).expect("bind");

    ss.listen(2).expect("listen");

    let (tss, _taddr) = ss.accept().expect("accept");

    for _ in 0..100 {
        let mut msg = [0u8; 2048];
        let len = tss.recv(&mut msg).expect("recv");

        let s = std::str::from_utf8(&msg[..len]).expect("Malformed message");

        println!("Got msg of len {} << {:?}", len, s);
    }

    ss.close().expect("close");

    cleanup().expect("cleanup");
}