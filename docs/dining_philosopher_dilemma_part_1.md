We will work on the dining philosopher's dilemma

# Initial Code

For the initial code, we analyzed deadlock conditions, making use of atomic-reference-counted shared pointers (`Arc`) and `Mutex` for thread-safe reference counting and mutual exclusion (so only one thread can access the data at a time).

We use `thread::spawn` to create concurrent threads. `Duration` is used to simulate time (thinking, eating, etc.). We also use `.map()` + `.collect()` to create a collection — specifically, a vector of forks.

---

## Rust Range Behavior

> > > Ranges in Rust are exclusive at the end, and indexing starts at 0.

So, for example, `0..5` includes: 0, 1, 2, 3, and 4 — but **not** 5.

---

## .map() and `|_|`

`.map()` takes each item in the range and transforms it into something else.

Now, the `|_|` part — that syntax is saying: "Hey, I don't care about the value from the range, I'm just doing the same thing for each item." The underscore `_` is a placeholder to indicate you're ignoring the value.

So, essentially we're telling Rust:

> Dude, create a variable called `forks`, it's a vector, and its length will match the number of philosophers. Each element in this vector will be a fork. We're going to wrap each fork in an `Arc`, and then inside a `Mutex`, because we want shared ownership, but also mutual exclusion.

We map the range into cloned, mutex-protected units and then `.collect()` all of them into the `forks` vector we already declared.

---

## Spawning Philosopher Threads

We're assigning a variable called `handles`, which is a vector created from mapping the range `0..PHIL_COUNT` — again, noting that the end is exclusive.

But this time, because we actually care about which philosopher we’re dealing with, we use `|i|` instead of `|_|`. `i` gives us access to the current index in the loop (which matches the philosopher's number, sort of — since index 0 is Philosopher 1, etc.).

Inside the mapping block:

* We create two variables: `left` and `right`.
* `left` is just `Arc::clone(&forks[i])`, grabbing the left fork.
* `right` is `Arc::clone(&forks[(i + 1) % PHIL_COUNT])`, which wraps around using `%` (modulo). This allows the last philosopher to grab fork 0 as their right fork. The modulo operator gives us the remainder, which is handy for looping back.

Then, we spawn a thread using `thread::spawn(move || { ... })`

---

## Closures and move keyword

Yes, `|| {}` is a closure.

The `move` keyword before it means: "Move ownership of all used variables into this closure." This is necessary because each closure will run in a separate thread, and Rust needs to ensure those variables are fully owned by the thread. This is safe, especially since we’re using `Arc`.

---

## What does format! do?

Inside the thread, we assign a name:

```rust
let name = format!("Philosopher {}", i + 1);
```

This just builds a string that looks like `Philosopher 1`, `Philosopher 2`, etc. We're not printing it yet — just storing it. Later, we use it with `println!()`.

---

## When does concurrency start?

Cloning the `Arc` allows us to *share* forks between threads, but the **actual concurrency begins** the moment we do this:

```rust
thread::spawn(...)
```

That's when the operating system spins up a new thread, and that thread starts doing its own thing in parallel.

And then finally:

```rust
let handles = (0..PHIL_COUNT).map(...).collect();
```

This collects all those `JoinHandle`s into a vector called `handles`, so we could `.join()` them later if we want (i.e., wait for them to finish).

We're essentially saying:

> Rust, go spin up a bunch of philosophers. Each one will think, get hungry, try to grab some forks, eat, and repeat — all in their own little world (aka thread).

