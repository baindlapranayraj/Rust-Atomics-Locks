# Chapter 1: Basics of Rust Concurrency

## Content Overview

This chapter is divided into two parts about Rust threads:

1. **Theory**
2. **Technical Code Explanation**

---

# Theory

## Processes 👾

- When you run a program, the Operating System (Kernel) creates a **process**.
- Each process is isolated from others, preventing free communication or data sharing between them.
- **Inter-Process Communication (IPC)** allows controlled data sharing between processes by requesting the OS (Kernel).

> **Note 📝:** Reasons for this design:
>
> 1. **Security:** Prevents malicious apps from accessing sensitive data (e.g., passwords) in other processes.
> 2. **Stability:** Ensures that a failure in one program doesn't crash the entire system.

---

## Threads 🧶

- In most current operating systems, an executed program’s code is run in a process, and the operating system will manage multiple processes at once. Within a program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads.
- Threads are smaller execution units within a process. A process can have multiple threads, all **sharing memory** within the same process. This splitting computation work in rust will increase the performance of application.
- Sharing memory between threads can cause issues like **race conditions** and **deadlocks**. Also, if one thread fails, it may affect other threads in the same process.
- By default, every process starts with one **main thread**. Additional threads can be created from this main thread.
- Each thread acts like an independent worker performing a specific task, enabling concurrent execution.

---

### Recommended Video

For a great visual explanation of processes and threads, check out the **Core Dump** channel:  
[Understanding Processes and Threads](https://www.youtube.com/watch?v=M9HHWFp84f0&t=62s)

---

# Technical Code Explanation

## Creating Threads 🧶

```
fn main() {
    spawn(f);   <=== Thread One created from the main thread to perform some computations.
    spawn(f);
    println!("Hello, world!🌍 from main thread");
}

fn f() {
    let id = current().id();
    println!("This is the Id of f thread {:?}", id);
}
```

In this above particular code if you try to run it, Your code spawns two threads and prints their IDs. However, sometimes the IDs don’t appear. This happens because the main thread exits before the spawned threads finish execution.

```
Hello, world!🌍 from main thread
```

### Why this happens ?

1. Threads in Rust are non-blocking by default. When you call spawn(f), Rust creates a new thread but doesn’t wait for it to finish. Returning from main will exit the entire program, even if other threads are still running.

2. The main function completes execution quickly. If the main thread finishes before the spawned threads have a chance to run, they may be terminated before printing their IDs.

3. The execution of thread is depends on how your operating system schedules the threads.

---

## Handling Threads 🧶:-

```
fn main() {
    println!("Hello, world!🌍 from main thread");

    let thread_a = spawn(f);
    let thread_b = spawn(f);

    thread_a.join().unwrap();   <=== added .join()   // Which means the main thread waits until the other threads are executed.
    thread_b.join().unwrap();
}

fn f() {
    let id = current().id();
    println!("This is the Id of f thread {:?}", id);

}
```

- The .join() method waits until the thread has finished executing and returns a std::thread::Result.
- Running this version of our program will no longer result in truncated output:

## Note (Internal behaviour of threads):-

> On a single-core processor, threads do not run truly in parallel but are time-sliced: the CPU switches rapidly between threads, giving the illusion of simultaneous execution which is know as **Concurrency**.
> Blocking a thread means that thread is prevented from performing work or exiting.

```
Hello, world!🌍 from main thread
This is the Id of f thread ThreadId(2)
This is the Id of f thread ThreadId(3)
```

---

### Scoped Threads:-

<h3>What are they ?</h3>
<h4>The function std::thread::scope allows us to create scoped threads, which means:</h4>

1. The child threads are guaranteed to finish before the main thread exits.

2. The threads can safely borrow local variables without requiring ownership transfer (move).

<h3>Why do we need them ?</h3>
<p>Normally, when using std::thread::spawn, Rust requires us to move ownership into the thread. This is because a spawned thread might outlive the function where it was created, causing potential dangling references.</p>

_The below code shows error due to borrowing in a normal thread (btw u can use the move for transfering the number ownership)_

```
use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    thread::spawn(|| {  // ❌ ERROR: Borrowing `numbers`
        for n in &numbers {
            println!("{n}");
        }
    }).join().unwrap();

    println!("Numbers: {:?}", numbers); // Main thread still needs `numbers`
}

```

**_💥 Error:_**

> These rust errors occures due to rust thinks spawned thread might outlives the main thread.

1. `thread::spawn` expects `'static data` (ownership or 'static references).
2. `numbers` is a local variable, and the thread might keep running after `main()` ends, making the reference invalid or dangling reference.

<h3>Solution is Scoped Threads</h3>

<p>If we know for sure that a spawned thread will definitely not outlive a certain scope, that thread could safely borrow things that do not live forever, such as local variables, as long as they outlive that scope. </p>

The Rust standard library provides the `std::thread::scope` function to spawn such scoped threads. It allows us to spawn threads that cannot outlive the scope of the closure we pass to that function, making it possible to safely borrow local variables.

<h4>Below is a Example of scoped thread</h4>

```
pub fn scoped_threads() {
    let num = vec![10, 2, 12, 55];

    thread::scope(|s| {  // 1
        s.spawn(|| { // 2
            println!("First spawn thread from scope function and we are accessing {num}");
        });

        s.spawn(|| {  // 2     <== We are able to read the num variable.
            let sum = num.iter().sum::<usize>();
            let res = sum / num.len();
            println!("The second thread of this scoope function is {res}");
        });
    }); // 3
}
```

> **_🤪 Fun Facts af :_**
>
> 1. The **Leakpocalypse** was a memory leak issue in Rust 1.0-beta due to scoped threads leaking references.
> 2. Rust removed scoped threads before Rust 1.0 to prevent this and introduced spawn threading.
> 3. Rust 1.63 reintroduced thread::scope, ensuring that borrowed variables remain valid until all threads finish.

## Memory Layout of a Running Program:

A program's memory is typically divided into these main segments:

<div>
 <img src="https://github.com/baindlapranayraj/Rust-Atomics-Locks/blob/main/chapter-1/images/rust-mem-layout.png?raw=true" alt="memory-layout">
</div>

## Sharing Data between threads :-

In Rust there are multiple ways to shares data between threads safely and it is fundamental part to know about concurrent programming.

### These are some approaches used for sharing data between threads:-

1. **_Using Arc<T> (Atomic Reference Counting)_**

- Arc<T> is a thread-safe reference-counted smart pointer that allows multiple threads to share ownership of immutable data.

- It enables multiple threads to read the same data concurrently.

- We cannot mutate the data of the given state of data, there is only read only, if you wanna mutate the data we need to use Mutex.

```
use std::sync::Arc;
use std::thread;

let numbers = Arc::new(vec![1, 2, 3]);
let mut handles = vec![];

for i in 0..3 {
    let numbers = Arc::clone(&numbers);
    let handle = thread::spawn(move || {
        println!("Number: {}", numbers[i]);   <== Only reference no mutation between threads
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}
```

2. Using Mutex<T> for Shared Mutable Access

- Mutex<T> provides mutual exclusion, allowing only one thread at a time to access (read or write) the protected data.

- Threads lock the mutex to access or modify the data, blocking if the lock is held by another thread.

- Due to this locking and unlocking nature the other threads get blocked(waited) this might cause a performance overhead but protects the data saving us from low-level errors.

```

use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));   <=== Generally used with Arc to share across all the threads.
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap());
```

3. Message Passing with Channels

- Rust encourages sharing memory by communicating rather than communicating by sharing memory.

- Channels (`std::sync::mpsc`) provide a way to send data between threads safely by **transferring ownership**.

- One thread sends data through a transmitter, and another receives it, avoiding shared mutable state.

```

use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("Hello from thread").unwrap();
});

println!("Received: {}", rx.recv().unwrap());

```
