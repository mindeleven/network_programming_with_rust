use tokio::net::{ TcpListener, TcpStream };
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::sync::mpsc;
use std::collections::HashMap;
use std::sync::{ Arc, Mutex };

type Tx = mpsc::UnboundedSender<String>; // type alias for the sender channel
type Rx = mpsc::UnboundedReceiver<String>; // type alias for the receiver channel

#[derive(Clone)]
struct Shared {
    peers: Arc<Mutex<HashMap<usize, Tx>>>
}

impl Shared {
    async fn broadcast(&mut self, sender_id: usize, message: &str) {
        
        let peers = self.peers.lock().unwrap();

        let mut to_remove = Vec::new();

        for (id, peer) in peers.iter() {
            if *id != sender_id {
                if let Err(_) = peer.send(message.into()) {
                    to_remove.push(*id); // mark disconnected clients for removal
                }
            }
        }
        for id in to_remove {
            self.peers.lock().unwrap().remove(&id); // remove disconnected clients
        }
    }
}

fn main() {
    println!("Hello, world!");
}
