/// borrowing: sharing without giving up control
/// - sharing a value without giving up ownership
/// - temporarily lending a value to another part of the code
/// two types of borrows:
/// (1) immutable borrows (&T)
///     -> borrower can only read, NOT modify
/// (2) mutable borrows (&mut T)
///     -> borrower can read AND modify
///     -> only one mutable borrow allowed at a time

// cargo run --bin ex05_borrowing

fn main() {
    
    let mut s: String = String::from("hello");
    
    let r1: &String = &s; // immutable borrow, no problem
    let r2 = &s; // another immutable borrow, no problem
    // both borrowed values can be used without any problem
    println!("Borrowed values r1 and r2: {}, {}", r1, r2);

    let r3 = &mut s; // mutable borrow, no problem
    // now we try another mutable borrow:
    // let r4 = &mut s; // causes error
    // error[E0499]: cannot borrow `s` as mutable more than once at a time
    // second mutable borrow occurs, first borrow later used
    // doesn't cause an error as long as I don't use r3 or r4
    // or try to use one of the immutable borrows afterwards

    // commenting out second mutable borrow
    // printing first mutable borrow
    println!("Mutable borrow r3: {}", r3);

}