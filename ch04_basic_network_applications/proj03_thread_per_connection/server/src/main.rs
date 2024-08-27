/// coding along with Network Programming with Rust by James L. Reid,
/// chapter 4, building basic network applications,
/// example: thread-per-connection echo server
/// 
/// echo server -> a parrot that repeats eveything you say
/// whenever it receives messages, it simply sends it back to the sender
/// tool for testing network connectivity
/// 
/// the SERVER

use std::{
    net::{
        TcpListener,
        TcpStream
    },
    io::{
        Read,
        Write
    },
    thread
};


fn main() {
    // bind the server to the local address and port
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Echo server listening on 127.0.0.1::8080");

    // using a for loop for continuously accepting incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // for each connection we call the handle_client function
                // to handle the client connection
                // handle_client(&mut stream); // from the simple echo server example

                // spawn a new thread for each client
                thread::spawn(move || {
                    handle_client(&mut stream);
                });
            },
            Err(e) => {
                println!("We've an error: {}", e);
            }
        }
    }
}

// handle the client connection
fn handle_client(stream: &mut TcpStream) {
    // creating a buffer to hold received data
    let mut buffer = [0; 1024];
    // and using a while loop to continuously read data from the client
    while match stream.read(&mut buffer) {
        Ok(size) => {
            // for each chunk of data received 
            // we echo it back to the client using stream.write_all
            // println!("{:?}", &stream);
            stream.write_all(&buffer[0..size]).unwrap();
            // continue loop as long as data being received
            true
        },
        Err(_) => {
            // println!(
            //     "An error occured, terminating connection with {}",
            //     stream.peer_addr().unwrap() // // failed to lookup address information: nodename nor servname provided, or not known
            // );
            println!(
                "An error occured, terminating connection"
            );
            false // break loop if error occurs
        }
    } {} // haven't seen this before ???
    

}