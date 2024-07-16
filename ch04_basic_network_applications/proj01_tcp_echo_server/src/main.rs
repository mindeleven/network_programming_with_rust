use std::{
    net::{
        TcpListener,
        TcpStream
    },
    io::{
        Read,
        Write
    }
};


fn main() {
    // bind the server to the local address and port
    let listener = TcpListener::bind("127.0.0.1::8080").unwrap();
    println!("Server listening on 127.0.0.1::8080");
}
