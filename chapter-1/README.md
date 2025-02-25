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
