/// Shared State with Mutexes: Collaboration with Guardrails
/// 
/// coding along with Network Programming with Rust by James L. Reid,
/// chapter 4, building basic network applications,


use std::{
    sync::{
        Arc, 
        Mutex
    },
    thread
};

/// example: using Arc (Atomic Reference Counting) 
/// -> to create a thread safe reference to a counter 
/// -> that can be shared accross multiple threads
/// 
/// a performance bottleneck might appear if multiple threads 
/// frequently need to access the shared data

fn main() {
    // create a shared counter
    let counter = Arc::new(Mutex::new(0));
    
    let mut handles = vec![];

    for _ in 0..10 {

        // clone the shared counter for each thread
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // lock the Mutex to access the counter
            // assuring only one thread can access it at a time
            let mut num = counter.lock().unwrap();
            // increment the counter
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Resut: {}", *counter.lock().unwrap());
}
