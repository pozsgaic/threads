//  EXAMPLE:  Transferring data into a thread.
use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 3, 2];
    let thread_handle = thread::spawn(move || {
        println!("Here's your vector: {:?}", v);
    });
    thread_handle.join().unwrap();
}
