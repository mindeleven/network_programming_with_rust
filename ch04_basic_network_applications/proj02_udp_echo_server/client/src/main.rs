use std::net::UdpSocket;

fn main() {
    // bind to available port
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

    // connect to the server
    socket.connect("127.0.0.1:8080").unwrap();

    let message = "Hello, UDP echo server!";
    // send the message
    socket.send(message.as_bytes()).unwrap();

    let mut buf = [0; 1024];
    // receive the echo
    let (num_bytes, _) = socket.recv_from(&mut buf).unwrap();

    println!("Received: {}", String::from_utf8_lossy(&buf[0..num_bytes]));
}
