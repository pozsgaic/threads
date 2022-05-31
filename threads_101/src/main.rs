use std::thread;
use std::time::Duration;
use log::debug;


fn main() {
    let thread_handle = thread::spawn(|| {
        for i in 1..100 {
            println!("Number from spawned thread: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..5 {
        println!("Number from main thread: {}", i);
        thread::sleep(Duration::from_millis(5));
    }

    //	Handle is of type JoinHandle 
    //  JoinHandle methods:  
    //  [
    //    thread(&self) -> &Thread
    //    join(&self)   -> Result<T>
    //    is_running(&self) -> bool
    thread_handle.join().unwrap();
}
