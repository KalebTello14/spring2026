// use std::thread;
// use std::time::Duration;

// fn main() {
//     println!("Main thread starting");
    
//     // Create a vector to store thread handles
//     let mut handles = vec![];
    
//     // Spawn 3 threads
//     for i in 1..=3 {
//         let handle = thread::spawn(move || {
//             println!("Thread {} starting", i);
//             thread::sleep(Duration::from_millis(500));
//             println!("Thread {} finished", i);
//         });
        
//         // Store the handle
//         handles.push(handle);
//     }
    
//     // Wait for all threads to complete
//     for handle in handles {
//         handle.join().unwrap();
//     }
    
//     println!("All threads completed.");
// }
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a shared counter using Arc and Mutex
    let counter = Arc::new(Mutex::new(0));
    
    let mut handles = vec![];
    
    // Spawn 5 threads
    for _i in 1..=5 {
        // Clone the Arc for each thread
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            // Increment counter 10 times
            for _ in 0..10 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Print the final value of the counter
    println!("Final counter value: {}", *counter.lock().unwrap());
}