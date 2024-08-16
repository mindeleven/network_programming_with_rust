use std::net::UdpSocket;

fn main() {
    // bind to localhost port 8080
    let socket = UdpSocket::bind("127.0.0.1:8080").unwrap();

    println!("Echo server listening to port 8080");

    let mut buf = [0; 1024]; // buffer to store oncoming data

    loop {
        // receive data and sender address
        let (num_bytes, src_addr) = socket.recv_from(&mut buf).unwrap();

        // echo the data back to the sender
        socket.send_to(&buf[0..num_bytes], src_addr).unwrap();
    }
}
