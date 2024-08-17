use std::net::UdpSocket;

fn main() {
    // (1) bind and connect
    // client crestes UdpSocket and binds it to any available port on local machine
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

    // after being bound socket connects to the server at 127.0.0.1:8080
    socket.connect("127.0.0.1:8080").unwrap();
    
    let message = "Hello, UDP echo server!";
    // (2) client sends the message to the server
    socket.send(message.as_bytes()).unwrap();
    
    // (3) client receives echoed message from the server
    let mut buf = [0; 1024];
    // receive the echo
    let (num_bytes, _) = socket.recv_from(&mut buf).unwrap();

    println!("Received: {}", String::from_utf8_lossy(&buf[0..num_bytes]));
}
