/// asynchronous Rust -> allows to write network applications that can effectively handle
/// multiple connections
/// example: fetching data from different websites
/// 
/// key advantages of using async Rust for network applications:
/// -> Improved Performance: 
///    app can handle multiple connections and requests simultaneously w/o blocking main thread
/// -> Simplified Concurrency:
///    async/await syntax makes writing readable asynchronous code easier
/// -> Resource Efficiency:
///    async I/O avoids need to create multiple threads
/// -> Enhanced Scalability:
///    async apps can scale more easily to increased traffic
/// -> Improved User Experience:
///    async apps tend to be more responsive with a smoother user experience

use std::time::Duration;
use tokio::time::sleep;

// tokio::main attribute tells the tokio runtime to execute main function asynchronously
#[tokio::main]
async fn main() {
    // creating two asyncronous tasks with tokio::spawn
    let task_1: tokio::task::JoinHandle<()> = tokio::spawn(async {
        // printing message that task is fetching data
        println!("Fetching data from website A....");
        // using tokio::time::sleep to simulate delay
        // await keyword pauses execution of each task until sleep function completes
        sleep(Duration::from_secs(2)).await;
        println!("Received data from website A....");
    });
    
    let task_2: tokio::task::JoinHandle<()> = tokio::spawn(async {
        println!("Fetching data from website B....");
        sleep(Duration::from_secs(2)).await;
        println!("Received data from website B....");
    });
    
    // waiting for the two tasks to complete
    task_1.await.unwrap();
    task_2.await.unwrap();

}
