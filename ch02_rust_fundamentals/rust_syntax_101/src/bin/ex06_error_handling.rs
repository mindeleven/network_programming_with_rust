// cargo run --bin ex06_error_handling

/// error handling in Rust
/// Rust uses the Result type and pattern matching to handle errors
/// the result type: a tale of two possibilities
/*
enum Result<T, E> {
    Ok(E),
    Err(E)
}
*/

use std::net::TcpStream;
use std::io::Write;

// ways of error handling in Rust

// (1) pattern matching
fn _match_result(result: Result<(), Box<dyn std::error::Error>>) {
    match result {
        Ok(value) => {
            // do something with the received data
            println!("{:?}", value)
        },
        Err(_) => {
            // we got an error, now what?
        }
    }
}

// (2) unwrapping
//     -> extracts the value from Ok(), panics if Err
fn _unwrap_result(result: Result<(), Box<dyn std::error::Error>>) {
    
    let value = result.unwrap();
    println!("{:?}", value);
}

// (3) error propagation
//     -> propagation errors up the call stack 
fn _error_propagation(filename: &str) -> Result<String, std::io::Error> {
    
    let contents = std::fs::read_to_string(filename)?;
    
    Ok(contents)
}

// another error propagation example
// function to send a message over a tcp connection
fn _send_message(stream: &mut TcpStream, message: &str) -> Result<(), std::io::Error> {
    
    stream.write_all(message.as_bytes())?; // will return std::io::Error if write_all fails
    
    Ok(())

}

fn main() {
    println!("Error handling in Rust with the Result type");
}
