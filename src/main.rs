use futures::future::ok;
use libc::stat;
use srt_rs::{self as srt};
use srt_rs::*;

use std::{io::Error, net::{SocketAddr, UdpSocket}};

//SRT Listener Receiver -> UDP Output
fn listener_receiver() -> std::io::Result<()> {
    let remote = "127.0.0.1:1234"; // args.next().unwrap();
    let output_addr: SocketAddr = "127.0.0.1:9090".parse().unwrap();

    println!("Listening {}", remote);
    let addr: SocketAddr = remote.parse().expect("Invalid addr:port syntax");

    //srt::startup().expect("startup");
    /*
    let ss = SrtSocket::new().expect("create_socket");

    ss.bind(addr).expect("bind");

    ss.listen(2).expect("listen");

    let (tss, _taddr) = ss.accept().expect("accept");    
    */
    
    let socket_output = UdpSocket::bind("0.0.0.0:0")?;

    let ss = srt::builder()
        .set_live_transmission_type()
        .listen(remote, 2).expect("asd");

    let (tss, _taddr) = ss.accept().expect("accept");
    println!("Accepted connection from {:?}", _taddr);
    println!("Socket Accepted on {:?}", ss.socket.get_socket_state());

    let mut is_connected = false;
    loop {
        
        let mut buffer = [0u8; 2048];
        let num_bytes: usize = tss.socket.recv(&mut buffer)?;

        let status = tss.socket.get_socket_state().expect("get_status");
        //let bis = tss.srt_bistats(0,1).expect("get_bistats");
        //println!("BISTATS: {:?}", bis);

        if status == SrtSocketStatus::Connected {
            if !is_connected {
                println!("Socket is connected");
            }
            is_connected = true;

            //socket_output.send_to(&buffer[..num_bytes], &output_addr)?;
            match socket_output.send_to(&buffer[..num_bytes], &output_addr) {
                Ok(_) => {},
                Err(e) => {
                    println!("Error sending to UDP: {}", e);
                }
            }
        }
        else {
            println!("Socket state: {:?}", status);
        }
    }
    
    println!("Exiting receiver loop");
    tss.close();
    Ok(())
}

// UDP Input -> SRT Listener sender
async fn listener_sender() -> std::io::Result<()>{
    let input_addr = "127.0.0.1:8080";
    // Dirección de salida UDP
    let output_addr: SocketAddr = "127.0.0.1:9090".parse().unwrap();

    // Crear un socket UDP para recibir datos
    let socket = UdpSocket::bind(input_addr)?;
    println!("Listening {}", input_addr);

    let mut buffer = [0; 65507];


    let ss = SrtSocket::new().expect("create_socket");
    // let nn = SrtAsyncBuilder::new()
    //     .set_socket_type(SrtSocketType::Listener)
    //     .set_stream_id(1)
    //     .set_max_connections(2)
    //     .build()
    //     .expect("create_socket");

    // let ss: Result<SrtAsyncListener, error::SrtError> = async_builder()
    //         .set_live_transmission_type()
    //         .listen(output_addr, 2, None);
            



    ss.bind(output_addr).expect("bind");

    ss.listen(2).expect("listen");

    let (tss, _taddr) = ss.accept().expect("accept");

    loop {
        // Recibir datos
        let num_bytes = socket.recv(&mut buffer)?;

        let status = tss.get_socket_state().expect("get_status");
        
        if status == SrtSocketStatus::Connected {
            //println!("Socket is connected");
            tss.send(&buffer[..num_bytes]).expect("send");
        }   
        else if status == SrtSocketStatus::Closed {
            println!("Socket is closed");
            //break; // Salir del bucle si el socket se desconecta
        } else {
            println!("Socket state: {:?}", status);                        
        }     
    }
}

//SRT Caller Receiver -> UDP Output
fn caller_receiver() -> std::io::Result<()>{
    let remote = "190.216.145.217:33214"; // args.next().unwrap();
    let output_addr: SocketAddr = "127.0.0.1:9090".parse().unwrap();

    let addr: SocketAddr = remote.parse().expect("Invalid addr:port syntax");
    /*
    let ss = SrtSocket::new().expect("create_socket");

    ss.connect(addr).expect("connect");

    
    */

    let ss = match srt::builder()
        .set_live_transmission_type()
        .connect(remote) {
            Ok(ss) => ss,
            Err(e) => {
                println!("Error connecting: {}", e);
                return Err(Error::new(std::io::ErrorKind::Other, "Error connecting"));
            }
        };

    let socket_output = UdpSocket::bind("0.0.0.0:0")?;
    println!("Connected to {}", remote);
    loop {
        let mut buffer = [0u8; 2048];
        match ss.socket.recv(&mut buffer) {
            Ok(n) => {
                // Data received successfully
                match ss.socket.get_socket_state() {
                    Ok(status) => {
                        if status == SrtSocketStatus::Connected {
                            //println!("Socket is connected");
                            match socket_output.send_to(&buffer[..n], &output_addr) {
                                Ok(_) => {},
                                Err(e) => {
                                    println!("Error sending to UDP: {}", e);
                                }
                            }
                        }   
                        else if status == SrtSocketStatus::Closed {
                            println!("Socket is closed");
                            //break; // Salir del bucle si el socket se desconecta
                        } else {
                            println!("Socket state: {:?}", status);                        
                        }     
                    }
                    Err(e) => {
                        println!("Error getting socket state: {}", e);
                    }
                }                   
            }
            Err(e) => {
                println!("Error receiving data: {}", e);
                return Ok(());
            }
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
    
    srt::startup().expect("startup");

    // loop {
    //     listener_receiver();
    //     std::thread::sleep(std::time::Duration::from_secs(1));       
    // }
        
    loop {
        caller_receiver();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    
    //listener_sender().expect("listener_sender");
    //caller_sender();

    srt::cleanup().expect("cleanup");
}