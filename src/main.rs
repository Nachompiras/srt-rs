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

    startup().expect("startup");

    let remote = "***REMOVED***";

    let addr: SocketAddr = remote.parse().expect("Invalid addr:port syntax");

    let ss = SrtSocket::new().expect("create_socket");

    ss.connect(addr).expect("connect");

    loop {
        for i in 0..100 {
            let mut msg = [0u8; 2048];
            let len = ss.recv(&mut msg).expect("recv");
    
            let status = ss.get_socket_state().expect("get_status");
            
            match status {
                SrtSocketStatus::Broken => {
                    println!("Socket is broken");
                    break;
                }
                SrtSocketStatus::Closed => {
                    println!("Socket is closed");
                    break;
                }
                SrtSocketStatus::Connected => {
                    //println!("Socket is connected");
                }
                SrtSocketStatus::Listening => {
                    println!("Socket is listening");
                } 
                SrtSocketStatus::Init => {
                    println!("Socket is Init");
                }
                SrtSocketStatus::Opened => {
                    println!("Socket is opened");
                }
                SrtSocketStatus::Closing => {
                    println!("Socket is closing");
                }
                SrtSocketStatus::NonExist => {
                    println!("Socket non exist");
                }
                SrtSocketStatus::Connecting => {
                    println!("Socket connecting");
                }
            }
        }
    }


    /*
    let _bin = args.next().unwrap();

    let remote = "127.0.0.1:1234"; // args.next().unwrap();

    let addr: SocketAddr = remote.parse().expect("Invalid addr:port syntax");

    startup().expect("startup");

    let ss = SrtSocket::new().expect("create_socket");

    ss.bind(addr).expect("bind");

    ss.listen(2).expect("listen");

    let (tss, _taddr) = ss.accept().expect("accept");

    loop {
        for _ in 0..100 {
            let mut msg = [0u8; 2048];
            let len = tss.recv(&mut msg).expect("recv");
    
            let status = tss.get_socket_state().expect("get_status");
            
            match status {
                SrtSocketStatus::Broken => {
                    println!("Socket is broken");
                    break;
                }
                SrtSocketStatus::Closed => {
                    println!("Socket is closed");
                    break;
                }
                SrtSocketStatus::Connected => {
                    println!("Socket is connected");
                }
                SrtSocketStatus::Listening => {
                    println!("Socket is listening");
                } 
                SrtSocketStatus::Init => {
                    println!("Socket is Init");
                }
                SrtSocketStatus::Opened => {
                    println!("Socket is opened");
                }
                SrtSocketStatus::Closing => {
                    println!("Socket is closing");
                }
                SrtSocketStatus::NonExist => {
                    println!("Socket non exist");
                }
                SrtSocketStatus::Connecting => {
                    println!("Socket connecting");
                }
            }
    
            //println!("Got msg of len {}", len);
        }
        
    }
    

    ss.close().expect("close");

    cleanup().expect("cleanup");
     */

}