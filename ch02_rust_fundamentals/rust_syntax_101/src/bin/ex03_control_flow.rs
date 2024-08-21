// cargo run --bin ex03_control_flow

use std::net::TcpStream;

fn connect_to_server(ip_addr: &str, port: u16) -> Result<TcpStream, std::io::Error> {
    TcpStream::connect((ip_addr, port))
}

fn main() {
    let message = "Let's write an example that uses a control flow";
    println!("{}", message);

    let mut retry_count = 0;
    let max_entries = 3;

    let false_ip_addr = "1192.168.1.100";

    loop {
        let connection_result: Result<TcpStream, std::io::Error> 
            = connect_to_server(false_ip_addr, 8080);
        
        if connection_result.is_ok() {
            break; // connection successful, exit the loop
        }

        retry_count += 1;

        println!("No connection established to {} on first attempt ({} of {}).", 
            false_ip_addr, retry_count, max_entries);

        if retry_count >= max_entries {
            // maximum retries reached, handle the error
            
            match connection_result {
                Ok(_stream) => {
                    // Connection successful! Use the stream to send/receive data
                },
                Err(e) => {
                    println!("No connection established, we got an error: {}", e);
                }
            }

            break;
        }

        // wait for a short period before retrying

    }

}
