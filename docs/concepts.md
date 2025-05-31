# Rust Concepts
    In the context of programming, Atomic = All or Nothing.
This prevents interference from other threads.

### Arc = Atomically-Reference-Counted shared pointer
> They may be stored in static variables, initialized using the constant initializer like AtomicBool::new, and are often used for lazy global initialization

## Memory Model for Atomic Accesses
> C++ uses an object-based memory model
> Rust uses an access-based memory model
    As such, translation is required. When C++ talks about "the value of an object", it means the resulting bytes obtained at read. 
    When C++ talks about "the value of an _Atomic_ object", it refers to the result of doing an atomic load.
A modification of an atomic object refers to an atomic store.
