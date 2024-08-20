/// asynchronous Rust -> allows to write network applications that can effectively handle
/// multiple connections
/// example: fetching data from different websites
/// 

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
