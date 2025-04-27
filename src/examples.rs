use libc::stat;
use srt_rs::*;

use std::{io::Error, net::{SocketAddr, UdpSocket}};

//SRT Listener Receiver -> UDP Output
fn listener_receiver() -> std::io::Result<()> {
    let remote = "127.0.0.1:1234"; // args.next().unwrap();
    let output_addr: SocketAddr = "127.0.0.1:9090".parse().unwrap();

    let addr: SocketAddr = remote.parse().expect("Invalid addr:port syntax");

    startup().expect("startup");

    let ss = SrtSocket::new().expect("create_socket");

    ss.bind(addr).expect("bind");

    ss.listen(2).expect("listen");

    let (tss, _taddr) = ss.accept().expect("accept");
    let socket_output = UdpSocket::bind("0.0.0.0:0")?;
    
    loop {
        
        let mut buffer = [0u8; 2048];
        let num_bytes: usize = tss.recv(&mut buffer).expect("recv");

        let status = tss.get_socket_state().expect("get_status");
        let bis = tss.srt_bistats(0,1).expect("get_bistats");
        println!("BISTATS: {:?}", bis);

        if status == SrtSocketStatus::Connected {            
            socket_output.send_to(&buffer[..num_bytes], &output_addr)?;
        }

    }
    
    ss.close().expect("close");
}

// UDP Input -> SRT Listener sender
fn listener_sender() -> std::io::Result<()>{
    let input_addr = "127.0.0.1:8080";
    // Dirección de salida UDP
    let output_addr: SocketAddr = "127.0.0.1:9090".parse().unwrap();

    // Crear un socket UDP para recibir datos
    let socket = UdpSocket::bind(input_addr)?;
    println!("Listening {}", input_addr);

    let mut buffer = [0; 65507];


    let ss = SrtSocket::new().expect("create_socket");

    ss.bind(output_addr).expect("bind");

    ss.listen(2).expect("listen");

    let (tss, _taddr) = ss.accept().expect("accept");

    loop {
        // Recibir datos
        let (num_bytes, src_addr) = socket.recv_from(&mut buffer)?;

        let status = tss.get_socket_state().expect("get_status");
        
        if status == SrtSocketStatus::Connected {
            //println!("Socket is connected");
            tss.send(&buffer[..num_bytes]).expect("send");
        }        
    }
}

//SRT Caller Receiver -> UDP Output
fn caller_receiver() -> std::io::Result<()>{
    let remote = "addr:port"; // args.next().unwrap();
    let output_addr: SocketAddr = "127.0.0.1:9090".parse().unwrap();

    let addr: SocketAddr = remote.parse().expect("Invalid addr:port syntax");

    let ss = SrtSocket::new().expect("create_socket");

    ss.connect(addr).expect("connect");

    let socket_output = UdpSocket::bind("0.0.0.0:0")?;

    loop {
        let mut buffer = [0u8; 2048];
        let num_bytes = ss.recv(&mut buffer).expect("recv");

        let status = ss.get_socket_state().expect("get_status");
        
        if status == SrtSocketStatus::Connected {
            //println!("Socket is connected");
            socket_output.send_to(&buffer[..num_bytes], &output_addr)?;
        } else {
            println!("Socket is not connected");
        }                
    }
}

//UDP Input -> Srt Caller Sender
fn caller_sender() -> std::io::Result<()> {
    let input_addr = "127.0.0.1:8080";
    
    let output_addr: SocketAddr = "127.0.0.1:9090".parse().unwrap();

    let socket = UdpSocket::bind(input_addr)?;
    
    let mut buffer = [0; 65507];

    loop {
        let ss = SrtSocket::new().expect("create_socket");

        match ss.connect(output_addr) {
            Ok(_) => {        
                loop {
        
                    let (num_bytes, _) = socket.recv_from(&mut buffer)?;
            
                    let status = ss.get_socket_state().expect("get_status");
                    
                    if status == SrtSocketStatus::Connected {
                        //println!("Socket is connected");
                        ss.send(&buffer[..num_bytes]).expect("send");
                    }
                    else if status == SrtSocketStatus::Closed {
                        println!("Socket is closed");
                        break; // Salir del bucle si el socket se desconecta
                    } else {
                        println!("Socket state: {:?}", status);            
                    }        
                }
            }
            Err(e) => {
                println!("Error to connect: {}", e);                
            }
        }; 
        //sleep and retry
        std::thread::sleep(std::time::Duration::from_secs(5));       
    }
    Ok(())
}


fn main() {
    
    startup().expect("startup");

    listener_receiver().expect("listener_receiver");
    //caller_receiver().expect("caller_receiver");
    //listener_sender().expect("listener_sender");
    //caller_sender();

    cleanup().expect("cleanup");
}