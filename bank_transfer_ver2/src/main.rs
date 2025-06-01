use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let account_a = Arc::new(Mutex::new(1000));
    let account_b = Arc::new(Mutex::new(1000));

    let mut handles = vec![];

    for _ in 0..1000 {
        let a = Arc::clone(&account_a);
        let b = Arc::clone(&account_b);

        let handle = thread::spawn(move || {
            let mut a_balance = a.lock().unwrap();

            if *a_balance > 0 {
                *a_balance -= 1;
                // simulate processing delay
                thread::sleep(Duration::from_micros(1));

                let mut b_balance = b.lock().unwrap();
                *b_balance += 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Balance A: {}", *account_a.lock().unwrap()); 
    println!("Final Balance B: {}", *account_b.lock().unwrap()); 
    println!("Total: {}", *account_a.lock().unwrap() + *account_b.lock().unwrap()); 
}
