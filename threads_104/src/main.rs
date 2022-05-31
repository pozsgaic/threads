// EXAMPLE:  Multiple producer single consumer thread example
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

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

    thread::spawn(move || {
        let values = vec![
            String::from("Too"),
            String::from("bad"),
            String::from("Bengals"),
        ];  
        for value in values {
            tx1.send(value).unwrap();
            //thread::sleep(Duration::from_millis(500));
        }   
    }); 

    for received in rx {
        println!("{}", received);
    }
}
