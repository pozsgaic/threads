// EXAMPLE:  Sharing data between threads with Mutex
// Data is shared with Arc -> atomic reference counted pointer
// Arc<T> is a shared pointer to object of type T
// It is typically used with a Mutex locking the value:
// Arc<Mutex<T>>

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn fibonacci(value: i32) -> i32 {
   match value {
      0 => 1,
      1 => 1,
      _ => fibonacci(value - 1) + fibonacci(value - 2)
   }
}

//  Mutex::lock()  returns LockResult<MutexGuard<'_, T>>
//  MutexGuard is RAII guard to allow scoped unlock of the lock meaning that
//  when the MutexGuard goes out of scope the mutex will be unlocked.
fn main() {
    let shared_value = Arc::new(Mutex::new(3));
    let mut handles = vec![];
    for _ in 1..10 {
        let shared_value = Arc::clone(&shared_value);
        let handle = thread::spawn(move || {
            let mut value = shared_value.lock().unwrap();
            println!("Value before fibonacci: {}", *value);
            *value = fibonacci(*value);
            println!("Value after fibonacci: {}", *value);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("RESULT:  {}", *shared_value.lock().unwrap());
}
