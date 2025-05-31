# Concurrency Concepts to Learn

## Overview

This document outlines the key concepts to learn for the report and project (Stage 1) in the Concurrent Programming course. The report covers race conditions, deadlocks, and deterministic finite automata (DFAs). The project focuses on implementing a sum program using Rust and Rayon, mimicking PVM’s master-slave model. The lists are ordered to guide your weekend work and streaming sessions.

---

## Report: Concepts to Learn (In Order)

1. **Race Conditions**  
   - **Description:** When multiple threads access shared data concurrently, and at least one modifies it, leading to unpredictable results.  
   - **Purpose:** Illustrate concurrency issues; demonstrate in Rust for the report.  
   - **Key Points:**  
     - Understand the critical section (code accessing shared data).  
     - Learn how unsynchronized access causes data races.  
     - Use Rust’s `Arc` and `Mutex` to prevent race conditions.  

2. **Deadlocks**  
   - **Description:** When two or more threads are stuck, each waiting for a resource another holds, causing a deadlock.  
   - **Purpose:** Show another concurrency pitfall with a Rust example.  
   - **Key Points:**  
     - Identify circular wait conditions (e.g., thread A holds lock X, wants Y; thread B holds Y, wants X).  
     - Understand how Rust’s `Mutex` can lead to deadlocks if misused.  

3. **Deterministic Finite Automata (DFA)**  
   - **Description:** A computational model with states, transitions, and an alphabet to process input strings.  
   - **Purpose:** Explain state transitions, relevant to concurrent systems; solve DFA exercises 1 and 2.  
   - **Key Points:**  
     - Learn DFA components: states, alphabet, transition function, start state, accept states.  
     - Practice solving DFA exercises (e.g., designing or analyzing state machines).  
     - Connect DFAs to concurrency (e.g., state transitions in thread coordination).  

4. **Concurrent Programming Concepts**  
   - **Description:** Principles of running multiple tasks simultaneously while avoiding issues like races and deadlocks.  
   - **Purpose:** Tie report concepts together and discuss their role in concurrency.  
   - **Key Points:**  
     - Compare shared-memory (e.g., Rust mutexes) vs. message-passing (e.g., PVM) concurrency.  
     - Explore trends in concurrency (e.g., Rust’s ownership model, actor systems).  

---

## Project (Stage 1): Concepts to Learn (In Order)

1. **Rust Basics (Review)**  
   - **Description:** Core Rust concepts from chapters 1-3 of the Rust manual.  
   - **Purpose:** Foundation for writing the sum program and using concurrency tools.  
   - **Key Points:**  
     - Review variables, functions, loops, and command-line argument handling (`std::env`).  
     - Understand ownership and borrowing basics for concurrency safety.  

2. **Sequential Programming**  
   - **Description:** Writing a program to sum numbers in a range without parallelism.  
   - **Purpose:** Create a baseline for comparison with the concurrent version.  
   - **Key Points:**  
     - Iterate over a range using a `for` loop or `Iterator::sum`.  
     - Parse command-line arguments with `std::env::args`.  
     - Measure execution time using `std::time::Instant`.  

3. **Rayon for Parallelism**  
   - **Description:** Rust library for data parallelism, simplifying concurrent tasks.  
   - **Purpose:** Replace PVM’s master-slave model for the concurrent sum program.  
   - **Key Points:**  
     - Add Rayon to `Cargo.toml` (`rayon = "1.10"`).  
     - Use `into_par_iter()` and `par_iter()` for parallel iteration.  
     - Understand how Rayon splits tasks across threads automatically.  

4. **Rust Concurrency Primitives**  
   - **Description:** Tools like threads, mutexes, and channels for concurrent programming.  
   - **Purpose:** Mimic PVM’s master-slave communication and reinforce race condition concepts.  
   - **Key Points:**  
     - Use `std::thread` for spawning threads.  
     - Use `std::sync::Arc` and `Mutex` for shared data.  
     - Use `std::sync::mpsc` (channels) for message-passing to mimic PVM.  

5. **Performance Profiling**  
   - **Description:** Measuring and comparing execution times of sequential vs. concurrent programs.  
   - **Purpose:** Demonstrate concurrency benefits for project documentation.  
   - **Key Points:**  
     - Use `std::time::Instant` for timing.  
     - Test with large ranges (e.g., 1 to 1,000,000).  
     - Document results (e.g., “sequential: 50ms, parallel: 20ms”).  

---

## Notes for Execution

- **Report:** Start with Rust code for race conditions and deadlocks (stream-friendly demos). Then solve DFA exercises and write the conclusion. Aim to complete by Saturday evening.  
- **Project:** Code the sequential sum program first, then add Rayon for parallelism. Use Sunday to explore threads/channels and document performance.  
- **Streaming:** Demo each concept (e.g., race condition bug/fix, Rayon speed-up, DFA diagrams). Highlight Rust’s ownership model as a concurrency feature.  
- **Setup:** Ensure Rust is installed (`rustup`), add Rayon (`cargo add rayon`), and use `cargo check` to catch errors early.