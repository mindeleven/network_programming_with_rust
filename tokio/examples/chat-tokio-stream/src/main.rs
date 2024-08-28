/// coding along with Creating a Chat Server with async Rust and Tokio by Lily Mara
/// video tutorial @ https://www.youtube.com/watch?v=T2mWg91sx-o
/// description from youtube:
/* "Building a chat server is a great way to learn the Tokio library because a chat server forces 
you to think about concurrent IO, which is the core purpose of Tokio. In this video, Lily 
demonstrates how you can spawn background tasks to manage independent network streams, and 
use tokio::select! to concurrently poll tasks which require a shared state. 
" */

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener
};

/// incremental steps to creating a chat sever
/// 1st step -> making a tcp echo server

#[tokio::main]
async fn main() {
    // 1st step -> making a tcp echo server
    // TcpListener::bind() returns a Future
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await // waiting to get the Result out of the Future
        .unwrap(); // unpacking the Result

    // accepting connections after we got the listener
    // -> calling accept() on the listener; returns a Future
    // -> socket which tcp stream & address which is socket address
    // after we got socket we can connect with `telnet localhost 8080`
    let (mut socket, _addr) = listener.accept().await.unwrap();
    
    // accepting an incoming message from the client
    let mut buffer = [0u8; 1024]; // setting up buffer with spave for 1024 bytes
    let bytes_read = socket.read(&mut buffer).await.unwrap(); // returns number of bytes that were read
    
    // sending read bytes back to the client
    socket.write_all(&buffer[..bytes_read]).await.unwrap();

    println!("Hello, world!");
}
