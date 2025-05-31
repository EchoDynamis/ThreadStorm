use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const PHIL_COUNT: usize =5;

fn main() {
    //create forks
    let forks: Vec<_> = (0..PHIL_COUNT)
        .map(|_| Arc::new(Mutex::new(())))
        .collect();

    // spawn philosopher threads
    let handles: Vec<_> = (0..PHIL_COUNT).map(|i| {
        let left = Arc::clone(&forks[i]);
        let right = Arc::clone(&forks[(i+1) % PHIL_COUNT]);

        thread::spawn(move || {
            let name = format!("Philosopher {}", i+1);
            loop {
                println!("{name} is thinking.");
                thread::sleep(Duration::from_millis(500));

                println!("{name} is hungry.");
                let _left_fork = left.lock().unwrap(); // lock left fork
                thread::sleep(Duration::from_millis(10)); // simulate reach time
                let _right_fork = right.lock().unwrap(); // lock right forl

                println!("{name} is eating.");
                thread::sleep(Duration::from_millis(1000));
            }

        })
        
    }).collect();

    // wait for all threads
    for h in handles {
        h.join().unwrap();
    }
}
