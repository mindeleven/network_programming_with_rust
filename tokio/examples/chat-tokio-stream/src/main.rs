/// coding along with Creating a Chat Server with async Rust and Tokio by Lily Mara
/// video tutorial @ https://www.youtube.com/watch?v=T2mWg91sx-o
/// description from youtube:
/* "Building a chat server is a great way to learn the Tokio library because a chat server forces 
you to think about concurrent IO, which is the core purpose of Tokio. In this video, Lily 
demonstrates how you can spawn background tasks to manage independent network streams, and 
use tokio::select! to concurrently poll tasks which require a shared state. 
" */

use tokio::{
    io::{ AsyncBufReadExt, AsyncWriteExt, BufReader },
    net::TcpListener, sync::broadcast
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
    
    // to accept multiple connections we need to pack everything into an extra loop
    loop {
        // accepting connections after we got the listener
        // -> calling accept() on the listener; returns a Future
        // -> socket which tcp stream & address which is socket address
        // after we got socket we can connect with `telnet localhost 8080`
        let (mut socket, _addr) = listener.accept().await.unwrap();

        // we need to communicate the line that's read from on client to every client connected
        // using a broadcast channel which ->
        // allows multiple producers and multiple consumers to send and receive on a single channel
        // channel gets number of items it can receive as a parameter
        let (tx, _rx) = broadcast::channel::<String>(10);

        // cloning tx before moving it into the loop
        let tx = tx.clone();
        // pulling the receiver out of the sender and not using the one we got above
        let mut rx = tx.subscribe();

        // we need to work with different tasks correctly so we need to spawn different tasks
        // passing an async block to tokio spawn
        tokio::spawn(async move {
        
            // separating the read part from the write part of the socket
            // necessary because the read part needs to be moved into BufReader and can't be used in the loop
            let (read, mut writer) = socket.split();

            // Using BufReader instead of creating a buffer to allow for a higher level of read operations
            let mut reader = BufReader::new(read);
            let mut line = String::new();

            // after we got a socket we can drop into an inifinite loop 
            // that allows to accept an infinite number of connections
            loop {
                // accepting an incoming message from the client
                // let mut buffer = [0u8; 1024]; // setting up buffer with spave for 1024 bytes

                
                // bytes_read with setting up a buffer:
                // let bytes_read: usize = socket.read(&mut buffer).await.unwrap(); // returns number of bytes that were read
                
                // bytes_read with BufReader:
                let bytes_read = reader.read_line(&mut line).await.unwrap();

                // bytes_read can tell us if the client has disconnected or not
                if bytes_read == 0 {
                    // reader has reached end of file and there's no data left
                    break;
                }

                // sending read bytes back to the client
                // sending with setting up a buffer:
                // socket.write_all(&buffer[..bytes_read]).await.unwrap();

                // now using the broadcast channel to send items on the channel
                tx.send(line.clone()).unwrap();

                // we also need to receive items on the channel
                let msg = rx.recv().await.unwrap();

                // sending with BufReader:
                // write is the Write half of the socket
                // writer.write_all(&line.as_bytes()).await.unwrap();
                
                // broadcasting the message to all clients
                writer.write_all(msg.as_bytes()).await.unwrap();

                // the BufReader adds line after line by default
                // so to just send the most current line back we need to clear it
                line.clear();
            }

        });
    }
    
}
