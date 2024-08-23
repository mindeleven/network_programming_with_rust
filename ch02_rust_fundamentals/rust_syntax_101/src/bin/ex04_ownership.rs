/// ownership, the guardian of memory

/// (1) single responsibility
///     -> only owner of value can modify or destroy it
/// (2) clear lifecycle
///     -> when owner goes out of scope value is dropped & memory freed
/// (3) move semantics
///     -> when assigned to a new variable, ownership is transferred

// cargo run --bin ex04_ownership

fn main() {
    
    let s1: String = String::from("hello"); // s1 owns the string
    
    // move
    let s2 = s1; // ownership is moved from s1 to s2

    // print!("{}", s1); // causes error ->
    // error[E0382]: borrow of moved value: `s1`
    // move occurs because `s1` has type `String`, which does not implement 
    // the `Copy` trait
    // value borrowed after move

    println!("Moved value: {}", s2);

}