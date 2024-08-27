/// coding along with Real-Time Rust: Building WebSockets with Tokio Tungstenite by Chris Hay
/// video tutorial @ https://www.youtube.com/watch?v=p1ES0apOwUw
/// description from youtube:
/* " => in this tutorial, chris shows you how to get started with websockets using rust, tokio 
and tokio tungenstenite.   
=> we will create a new websocket client in rustlang using tokio and tokio-tungstenite, connect
it to a public server, and then be able to send and receive messages to a websocket server.
=> we will also connect up the rust websocket client to an existing ai agent network that chris 
has been creating (developed in a compeltely different technology, bun).   
=> we will then convert the application to becoming an automatic time bot on the ai agent network.
=> at the end of this video, you'll have a deep understanding on how to create web socket clients 
using rust" */
/* dependencies: 
cargo add tokio -F "tokio/full" futures-util tokio-tungstenite -F "tokio-tungstenite/native-tls"
*/

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() {
    // url to public websocket server
    // TODO: give this a try in postman
    let url: &str = "wss://echo.websocket.events";
    println!("Connecting to - {}", url);

    // connecting asynchronously to server
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to agent network");

    // split up the websocket stream into a write and read stream
    let (mut write, mut read) = ws_stream.split();
    
    // read before write
    if let Some(message) = read.next().await {
        let message = message.expect("Failed to read the message");
        println!("Reveived a message: {}", message);
    } 

    // sending a message
    let msg = Message::Text("Aloha echo server".into());
    println!("Sending message: {}", msg);
    write.send(msg).await.expect("Failed to send message");

    // reading the message and write it to the console

    // taking what comes out of this stream and read it into a message
    if let Some(message) = read.next().await {
        let message = message.expect("Failed to read the message");
        println!("Reveived a message: {}", message);
    } // reading the message from the stream
}