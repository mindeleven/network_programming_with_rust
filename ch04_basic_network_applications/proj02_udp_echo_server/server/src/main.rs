// importing the UdpSocket type from std::net
use std::net::UdpSocket;

fn main() {
    // creating UdpSocket, binding it to the local address
    // bind to localhost port 8080
    // server will listen for incoming udp packets on port 8080
    let socket = UdpSocket::bind("127.0.0.1:8080").unwrap();

    println!("Echo server listening to port 8080");

    let mut buf = [0; 1024]; // buffer to store oncoming data

    loop {
        // server enters infinite lopp to listen contiously for incoming packets

        // socket.recv_from -> receives UDP packet, stores it in buf buffer
        // receive data and sender address
        // returns number of bytes received and source address
        let (num_bytes, src_addr) = socket.recv_from(&mut buf).unwrap();
        
        println!("Echoing: {}", String::from_utf8_lossy(&buf[0..num_bytes]));

        // echo the data back to the sender at src_addr
        socket.send_to(&buf[0..num_bytes], src_addr).unwrap();
    }
}
