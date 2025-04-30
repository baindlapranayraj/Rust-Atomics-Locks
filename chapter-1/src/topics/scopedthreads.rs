use std::thread;
use sysinfo::System;

// +++++++++++++++++++ This section covers about scoped threads ++++++++++++++++++++

pub fn scoped_threads() {
    // Shared mutable vector
    #[allow(unused_mut)]
    let mut num = vec![10, 2, 12, 55];

    // Create a thread scope where all spawned threads must finish before continuing
    thread::scope(|s| {
        // ===========================
        // Problem 1: Mutable borrow in multiple threads
        // Uncommenting the below code causes a compile error because
        // Rust forbids multiple mutable borrows at the same time.
        //
        // for _ in 0..10 {
        //     s.spawn(|| {
        //         num.push(10); // Error: cannot borrow `num` as mutable more than once
        //     });
        // }
        //
        // Cloning `num` inside the closure would avoid the borrow error,
        // but then threads would not modify the original vector.
        //
        // Atomic types only work for primitive values, not complex types like Vec.
        //
        // The solution to safely mutate shared data across threads is covered in `thread_safety.rs`.
        // ===========================

        // Example thread: Calculate sum and average of `num`
        s.spawn(|| {
            let sum = num.iter().sum::<usize>();
            let res = sum / num.len();
            println!("The second thread of this scope function is {res}");
        });

        // Thread to print process details
        s.spawn(|| {
            println!("+++++++++ Process Details ++++++++++++");

            let mut system = System::new();
            system.refresh_all();

            let processes = system.processes();

            for (index, (pid, process)) in processes.iter().enumerate() {
                println!("\n====== Process #{} ======", index);
                println!("Name: {:?}", process.name());
                println!("PID: {}", pid);
                println!("CPU usage: {}%", process.cpu_usage());
                println!("Memory: {} bytes", process.memory());
                println!("Status: {:?}", process.status());
                println!("Command: {:?}", process.cmd());
            }

            println!("Total processes: {}", processes.len());
        });
    }); // thread::scope waits here until all threads finish
}

// ++++++++++++++++++++++++++++ Learnings ++++++++++++++++++++++++++++
//
// 1. `thread::scope` creates a scope where all spawned threads must complete
//    before the program continues. This ensures safe access to stack data.
//
// 2. Mutable borrowing rules in Rust prevent multiple mutable borrows,
//    which causes errors if you try to mutate shared data like `num`
//    concurrently without synchronization.
//
// 3. Cloning data inside threads avoids borrow errors but works on copies,
//    not the original data.
//
// 4. Atomic types only support primitive data, so for complex types like Vec,
//    you need synchronization primitives (Mutex, RwLock) to safely share mutable data.
//
// 5. `thread::scope` automatically joins all spawned threads at the end of the scope,
//    preventing dangling references and ensuring thread completion.
//
// 6. For safely sharing and mutating data across threads, see solutions involving
//    synchronization primitives (Mutex, Arc) in `thread_safety.rs` or similar.
//
// +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
