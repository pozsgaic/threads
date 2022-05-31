//  EXAMPLE:  Single producer single receiver message passing between threads

use std::thread;
use std::time::Duration;
// Multiple publisher, single consumer
// Has two halves ->  tx and rx.  Transmitter and Receiver.
use std::sync::mpsc;  

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let values = vec![
            String::from("Here"),
            String::from("we"),
            String::from("go"),
            String::from("Steelers"),
        ];
        for value in values {
            tx.send(value).unwrap();
            //thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println!("{}", received);
    }
}
