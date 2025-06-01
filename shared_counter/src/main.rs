#![allow(static_mut_refs)]

use std::thread;

static mut SHARED_COUNTER: i32 = 0;

fn main() {
    let handles: Vec<_> = (0..100)
        .map(|_| {
            thread::spawn(|| {
                for _ in 0..1000 {
                    unsafe {
                        SHARED_COUNTER += 1;
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        println!("Final counter: {}", SHARED_COUNTER);
    }
}

