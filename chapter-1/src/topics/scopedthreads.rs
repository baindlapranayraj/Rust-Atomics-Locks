use std::thread;

use sysinfo::System;

pub fn scoped_threads() {
    let mut num = vec![10, 2, 12, 55];
    let mut lang = vec!["GO 🦝", "Rust 🦀", "Typescript"];

    thread::scope(|s| {
        // 1
        s.spawn(|| {
            // 2
            // println!(
            //     "First spawn thread from scope function we are accesing {:?}",
            //     num
            // );
        });

        s.spawn(|| {
            // 2
            let sum = num.iter().sum::<usize>();
            let res = sum / num.len();
            // println!("The second thread of this scoope function is {res}");
        });

        s.spawn(|| {
            let mut system = System::new();

            system.refresh_all();
            println!("+++++++ CPU Details +++++++++");

            let cpus = system.cpus();

            for cpu in cpus {
                println!(
                    "The CPU name: {}, CPU brand: {}, CPU usage: {}",
                    cpu.name(),
                    cpu.brand(),
                    cpu.cpu_usage()
                );
            }
        });

        s.spawn(|| {
            println!("+++++++++ Process Details ++++++++++++");

            let mut system = System::new();

            system.refresh_all();
            let process = system.processes();

            for (index, (pid, process)) in process.iter().enumerate() {
                println!("====== Process #{} ======", index);
                println!("Name: {:?}", process.name());
                println!("PID: {}", process.pid());
                println!("CPU Usage: {}%", process.cpu_usage());
                println!("Memory: {} bytes", process.memory());
                println!("Status: {:?}", process.status());
                println!("Command: {:?}", process.cmd());
                println!();
            }
        });
    }); // 3
}

// ++++++++++++++++++++++++++++ Learnings ++++++++++++++++++++++++++++
// 1 ==> thread::scope is used to create a scope where all threads must finish before continuing.
//  The closure |s| { ... } defines the scope where we can spawn multiple threads.
//
// 3 ==> Ensuring All Threads Finish Before Continuing.
//
// When thread::scope ends, it waits for all spawned threads to complete execution before proceeding.
// This prevents dangling references and ensures safe memory access.
