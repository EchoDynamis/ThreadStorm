# Bank Transfer Simulation – Race Condition

This version simulates bank transfers between two accounts, using threads without proper synchronization. The goal is to intentionally create a race condition, where multiple threads access and modify shared data at the same time, leading to unpredictable results.

## What Is a Race Condition?

A race condition occurs when:

Two or more threads access shared data concurrently.

At least one of them modifies the data.

And there's no mechanism (like a lock) to coordinate the access.

This leads to unreliable or inconsistent results — the final value depends on who "gets there first," which in multi-threaded environments is non-deterministic.

## The Setup: Accounts and Transfers

We define two accounts as integer values. In this version:

static mut ACCOUNT_A: i32 = 1000;
static mut ACCOUNT_B: i32 = 1000;

They both start at $1000.

We then simulate concurrent transfers between these two accounts by spawning multiple threads. Each thread will:

Withdraw a small amount from ACCOUNT_A.

Deposit that amount into ACCOUNT_B.

Because we're not using any Mutex, the memory is shared unsafely — and that’s the whole point. We’re showcasing how things can go wrong.

##  Threads and Transfers

We create threads using:

let handles: Vec<_> = (0..100)
    .map(|_| {
        thread::spawn(|| {
            for _ in 0..1000 {
                unsafe {
                    ACCOUNT_A -= 1;
                    ACCOUNT_B += 1;
                }
            }
        })
    })
    .collect();

Let’s break that down:

let handles = (0..100)

We’ll spawn 100 threads. Each thread will perform the transfer 1000 times.

.map(|_| thread::spawn(...))

We use .map() with _ because we don't care about the index — we're doing the same thing in every thread. The thread::spawn part launches each thread.

unsafe { ACCOUNT_A -= 1; ACCOUNT_B += 1; }

This is the critical section: the shared data (accounts) is being read and modified without protection. This is what causes a race condition.

## What's Going Wrong?

At the end of the program, we expect:

$100,000 to have been transferred in total.

So ACCOUNT_A should be 0 and ACCOUNT_B should be 2000.

But in reality, because of unsynchronized access, threads overwrite each other’s progress. Some increments and decrements are lost due to simultaneous writes.

So the final output might be:

ACCOUNT_A: 97
ACCOUNT_B: 1993

Or some other inaccurate result. It changes every time.

## Thread Joining

At the end, we use:

for handle in handles {
    handle.join().unwrap();
}

This waits for all the threads to finish. Without this, the main thread might exit early before the others complete their transfers.

## Summary of Critical Points

This example intentionally omits synchronization to demonstrate a race condition.

The unsafe block is required in Rust to mutate a static mut variable — but using it this way is a bad idea unless it’s tightly controlled (which we’re not doing here).

Threads operate on shared memory without locks, leading to data races.

The final output is non-deterministic and varies each time.

### This simulation helps us observe what a race condition looks like in a real-world context (e.g., bank systems), and prepares us to fix it later using synchronization primitives like Mutex and Arc.


