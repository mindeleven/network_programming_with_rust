// cargo run --bin ex02_real_world_functions

use std::net::TcpStream;

fn connect_to_server(ip_addr: &str, port: u16) -> Result<TcpStream, std::io::Error> {
    TcpStream::connect((ip_addr, port))
}

fn main() {
    let message = "Let's write a Real World Function in Rust and connect to a server via TCP";
    println!("{}", message);

    let connection_result = connect_to_server("192.168.1.100", 8080);

    match connection_result {
        Ok(_stream) => {
            // Connection successful! Use the stream to send/receive data
        },
        Err(e) => {
            println!("No connection established, we got an error: {}", e);
        }
    }

}
