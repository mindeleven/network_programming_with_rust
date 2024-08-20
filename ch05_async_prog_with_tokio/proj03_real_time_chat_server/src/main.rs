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

    async fn handle_connection(mut socket: TcpStream, id: usize, mut shared: Shared) {
        
        let (tx, mut rx) = mpsc::unbounded_channel();

        shared.peers.lock().unwrap().insert(id, tx);

        let (mut reader, mut writer) = socket.split();

        let mut buf = [0; 1024];

        loop {
            tokio::select! {
                result = reader.read(&mut buf) => {
                    match result {
                        Ok(0) => {
                            println!("Client {} disconnected", id);
                            shared.peers.lock().unwrap().remove(&id);
                            return;
                        },
                        Ok(n) => {
                            let message = String::from_utf8_lossy(&buf[0..n]);
                            shared.broadcast(id, &message).await;
                        },
                        Err(e) => {
                            eprintln!("error: {}", e);
                            return;
                        },
                    }
                }
                Some(message) = rx.recv() => {
                    if let Err(e) = writer.write_all(message.as_bytes()).await {
                        eprintln!("error: {}", e);
                        return;
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on port 8080");

    let shared = Shared {
        peers: Arc::new(Mutex::new(HashMap::new())),
    };
    
    let mut next_id = 1;

    loop {

        let (socket, _) = listener.accept().await.unwrap();

        println!("New client connected");

        let shared_clone = shared.clone();

        tokio::spawn(async move {
            Shared::handle_connection(socket, next_id, shared_clone).await;
        });

        next_id += 1;

    }
}
