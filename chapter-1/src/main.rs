use std::thread::{current, spawn};

fn main() {
    println!("Hello, world!🌍 from main thread");

    let thread_a = spawn(f);
    let thread_b = spawn(f);

    thread_a.join().unwrap();
    thread_b.join().unwrap();

    let langs = vec!["Typescript 💪🏻","Go 🦝","Rust 🦀"];


    spawn( move || {   // `move` forces ownership transfer
        for name in langs {
            println!("The programming langauge is {name}");
        }     
    }).join().unwrap();

}

fn f() {
    let id = current().id();
    println!("This is the Id of f thread {:?}", id);

    // Hear the main thread is the threadId(1)
}


// Notes:- 
// 1) Hear our code spawns two threads and prints their IDs. However, sometimes the IDs don’t appear. This happens because 
//    the main thread exits before the spawned threads finish execution.
// 
// 2) By default, closures capture variables by reference (&langs).