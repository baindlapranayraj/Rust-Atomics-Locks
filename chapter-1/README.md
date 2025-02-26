# Chapter 1. Basics of Rust Concurrency

## Content Overview
This chapter is divided into two parts:
1. Theory
2. Technical code explanation
----

# Theory 
## Processes 👾

- When you run a program, the Operating System (Kernel) creates a process. Each process is isolated from others, preventing free communication or data sharing between them.

- Inter-Process Communication (IPC) allows controlled data sharing between processes by requesting the OS (Kernel).


> **Note 📝**: Reasons for this design:
> 1. **Security**: Prevents malicious apps from accessing sensitive data (e.g., passwords) in other processes.
> 2. **Stability**: Ensures that a failure in one program doesn't crash the entire system.


## Threads 🧶

- Threads are smaller execution units within a process. A process can have multiple threads, all sharing memory within the same process.

- Due to sharing the same space between the threads might cause some issues like:- race conditions, deadlock etc and also if one thread fails to execute then all of the threads with in the shared space(process) will fail.

- By default, every process starts with one main thread. Additional threads can be created from this main thread.

- Each thread is like a independent worker to do some specific task and using multiple threads you can handle multiple task concurrently. 

---

# Technical Code Explanation

## Creating Threads 🧶:-
```
fn main() {
    spawn(f);
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
1) Threads in Rust are non-blocking by default. When you call spawn(f), Rust creates a new thread but doesn’t wait for it to finish.   Returning from main will exit the entire program, even if other threads are still running.

2) The main function completes execution quickly. If the main thread finishes before the spawned threads have a chance to run, they may be terminated before printing their IDs.

---

## Handling Threads 🧶:- 

```
fn main() {
    println!("Hello, world!🌍 from main thread");

    let thread_a = spawn(f);
    let thread_b = spawn(f);

    thread_a.join().unwrap();   <=== added .join()
    thread_b.join().unwrap();
}

fn f() {
    let id = current().id();
    println!("This is the Id of f thread {:?}", id);

}

```

- The .join() method waits until the thread has finished executing and returns a std::thread::Result.
- Running this version of our program will no longer result in truncated output:



```
Hello, world!🌍 from main thread
This is the Id of f thread ThreadId(2)
This is the Id of f thread ThreadId(3)
```
---

### Scoped Threads:- 

<h3>What are they ?</h3>
<h4>The function std::thread::scope allows us to create scoped threads, which means:</h4>

1) The child threads are guaranteed to finish before the main thread exits.

2) The threads can safely borrow local variables without requiring ownership transfer (move).

<h3>Why do we need them ?</h3>
<p>Normally, when using std::thread::spawn, Rust requires us to move ownership into the thread. This is because a spawned thread might outlive the function where it was created, causing potential dangling references.</p>

*The below code shows error due to borrowing in a normal thread (btw u can use the move for transfering the number ownership)*

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


***💥 Error:***
> These rust errors occures due to rust thinks spawned thread might outlives the main thread.
1) ```thread::spawn``` expects ```'static data``` (ownership or 'static references).
2) ```numbers``` is a local variable, and the thread might keep running after ```main()``` ends, making the reference invalid or dangling reference.

<h3>Solution is Scoped Threads</h3>
<p>If we know for sure that a spawned thread will definitely not outlive a certain scope, that thread could safely borrow things that do not live forever, such as local variables, as long as they outlive that scope. </p>

The Rust standard library provides the ```std::thread::scope``` function to spawn such scoped threads. It allows us to spawn threads that cannot outlive the scope of the closure we pass to that function, making it possible to safely borrow local variables.

<h4>Below is a Example of scoped thread</h4>

```
pub fn scoped_threads() {
    let num = vec![10, 2, 12, 55];

    thread::scope(|s| {  // 1
        s.spawn(|| { // 2
            println!("First spawn thread from scope function and we are accessing {num}");
        });

        s.spawn(|| {  // 2
            let sum = num.iter().sum::<usize>();
            let res = sum / num.len();
            println!("The second thread of this scoope function is {res}");
        });
    }); // 3
}
```

> ***🤪 Fun Facts af :***
> 1) The **Leakpocalypse** was a memory leak issue in Rust 1.0-beta due to scoped threads leaking references.
> 2) Rust removed scoped threads before Rust 1.0 to prevent this and introduced spawn threading.
> 3) Rust 1.63 reintroduced thread::scope, ensuring that borrowed variables remain valid until all threads finish.

## Memory Layout of a Running Program:
A program's memory is typically divided into these main segments:

<div>
 <img src="chapter-1/images/rust-mem-layout.png" alt="memory-layout">
</div>