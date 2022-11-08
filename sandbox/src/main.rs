use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let sleep_dur: f64 = rand::random();
            thread::sleep(Duration::from_secs_f64(sleep_dur));
            let mut num = counter.lock().unwrap();



            *num = i;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());


    let a: i16 = 25;
    let b: u8 = a as u8;

    println!("{:?}",a);
    println!("{:?}",b);

}
