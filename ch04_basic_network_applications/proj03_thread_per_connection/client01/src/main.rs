/// coding along with Network Programming with Rust by James L. Reid,
/// chapter 4, building basic network applications,
/// mini-project 1: crafting your first TCP echo server and client
/// 
/// echo server -> a parrot that repeats eveything you say
/// whenever it receives messages, it simply sends it back to the sender
/// tool for testing network connectivity
/// 
/// the CLIENT
/// 
/// creating a client that can connect to our server and send messages

use std::{
    net::TcpStream,
    io::{
        Read,
        Write
    }
};

fn main() {
    // connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let mut count = 0;
    loop {
        let message = format!("Hello from the client 01 (take {}) :)", count);
        // send the message
        stream.write_all(message.as_bytes()).unwrap(); 
        
        // create a buffer to hold the response
        let mut buffer = [0; 1024];
        // read the response
        stream.read(&mut buffer).unwrap();

        // print the echoed message
        println!("Server echoed: {}", String::from_utf8_lossy(&buffer[..]));
        
        if count == 100 {
            break
        }

        count += 1;
    }
}

