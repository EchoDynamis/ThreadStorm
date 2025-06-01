#![allow(static_mut_refs)]

use std::thread;
use std::time::Duration;

// shared mutable state
static mut ACCOUNT_A: i32 = 1000;
static mut ACCOUNT_B: i32 = 1000;

fn main() {
    let mut handles = vec![];

    for _ in 0..1000 {
        let handle = thread::spawn(|| {
            // simulate a race condition: transfer $1 from A to B
            unsafe {
                if ACCOUNT_A > 0 {
                    ACCOUNT_A -= 1;
                    // simulate processing delay
                    thread::sleep(Duration::from_micros(1));
                    ACCOUNT_B +=1;
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        println!("Final Balance A: {}", ACCOUNT_A);
        println!("Final Balance B: {}", ACCOUNT_B);
        println!("Total: {}", ACCOUNT_A + ACCOUNT_B);
    }
 
}

