// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.



use std::sync::mpsc;
use std::sync::{Arc, Mutex};  
use std::thread;  
use std::time::Duration;  
  
struct JobStatus {  
    jobs_completed: Mutex<u32>, // use Mutex to protect shared data
}  
  
fn main() {  
    let status = Arc::new(JobStatus { jobs_completed: Mutex::new(0) });  
    let mut handles = vec![];  
    for _ in 0..10 {  
        let status_shared = Arc::clone(&status);  
        let handle = thread::spawn(move || {  
            thread::sleep(Duration::from_millis(250));  
            *status_shared.jobs_completed.lock().unwrap() += 1;  
        });  
        handles.push(handle);  
    }  
    for handle in handles {  
        handle.join().unwrap();  
    }  
    println!("jobs completed {}", *status.jobs_completed.lock().unwrap());  
}