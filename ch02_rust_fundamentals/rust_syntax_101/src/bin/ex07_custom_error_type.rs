#![allow(dead_code, unused_imports)]
/// defining a custom error type
/// to represent specific error conditions
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct NetworkError {
    kind: NetworkErrorKind,
    message: String
}

#[derive(Debug)]
enum NetworkErrorKind {
    ConnectionRefused,
    Timeout,
    InvalidData
}

/// TODO: implement the Error trait for NetworkError

fn main() {
    println!("Defining a custom error type");
}


