# Bank Transfer Example - Version 2: Race Condition Prevention with Mutex and Arc. 

## Imports
std::sync::{Arc, Mutex}:
We bring in two essential concurrency tools from Rust’s standard library.

Arc stands for Atomic Reference Counted pointer — it allows us to share ownership of data safely across threads by keeping track of how many owners exist.

Mutex is a mutual exclusion primitive — it ensures that only one thread can access the data inside it at any given time, preventing race conditions.

std::thread:
For creating and managing threads, enabling concurrency.

std::time::Duration:
To simulate a small delay in the processing, making the race condition window visible.

## Creating Shared Accounts with Arc and Mutex
let account_a = Arc::new(Mutex::new(1000));
let account_b = Arc::new(Mutex::new(1000));
We create two bank accounts, each with an initial balance of 1000.

These balances are wrapped inside a Mutex to allow safe mutable access from multiple threads — only one thread can hold the lock at a time.

Then, we wrap each mutex in an Arc so that the ownership of these mutex-protected balances can be safely shared between threads.

## Preparing to Spawn Threads
let mut handles = vec![];
We declare a mutable vector called handles that will store the handles to all threads we spawn.

These handles let us join the threads later, ensuring all threads complete their execution before moving on.

## Spawning 1000 Threads to Transfer Money
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
We loop 1000 times, each iteration simulating a transfer of 1 unit from account_a to account_b.

Inside each iteration:

We clone the Arcs for account_a and account_b. This increments the reference count safely and lets the new thread own these references.

We spawn a new thread with thread::spawn.

The closure passed to spawn is marked with move to take ownership of the clones of the accounts. This ensures data ownership is correctly transferred into the thread for safe concurrency.

Inside the thread:

We lock account_a’s mutex to get mutable access to the balance. lock() returns a MutexGuard, which dereferences to the inner integer.

We check if account_a has more than zero units before transferring money.

If yes, we decrement account_a by 1.

We simulate a tiny delay (sleep) to make the transfer process visible and allow race conditions to potentially show (if we didn't use locks).

Then we lock account_b’s mutex, safely incrementing its balance by 1.

Each thread handle is stored in the handles vector for later joining.

## Joining Threads to Wait for Completion
for handle in handles {
    handle.join().unwrap();
}
We iterate through all the thread handles and call .join() on each one.

Joining means we wait for the thread to finish before continuing.

.unwrap() is used to catch any panics or errors (if the thread panicked), so the main thread can handle it or exit cleanly.

## Printing Final Account Balances
println!("Final Balance A: {}", account_a.lock().unwrap());
println!("Final Balance B: {}", account_b.lock().unwrap());
After all transfers finish, we lock each account's mutex one last time to safely access and print the final balances.

If all transfers succeeded, account_a should be 0 and account_b should be 2000 — since 1000 units were moved from A to B.

This shows the race condition has been prevented by locking, guaranteeing consistent state.

Key Points
Race Condition Avoidance:
Without using Mutex to guard the balances, concurrent increments/decrements could cause lost updates — the infamous race condition. Here, locking prevents multiple threads from accessing the same balance simultaneously.

Arc for Shared Ownership:
Arc lets multiple threads own the same Mutex wrapped data safely.

Locks are Blocking:
Threads wait for the mutex lock before proceeding, enforcing mutual exclusion in the critical section — the balance modification code.

Thread Safety:
Rust’s ownership and concurrency model forces you to explicitly manage shared mutable state with safe abstractions (Arc<Mutex<T>>), helping avoid data races and undefined behavior.

### Conclusion
This version effectively demonstrates how to safely share and mutate data between threads using Rust’s concurrency tools. It highlights the concept of critical sections (the locked balance modification) and how mutual exclusion is necessary to avoid race conditions — which otherwise cause unpredictable and incorrect behavior in concurrent programs.
