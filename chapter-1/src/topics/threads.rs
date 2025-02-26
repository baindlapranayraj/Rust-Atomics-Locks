use std::thread::{current, spawn};

pub fn main_thread() {
    println!("Hello, world!🌍 from main thread");

    let thread_a = spawn(f);
    let thread_b = spawn(f);

    thread_a.join().unwrap();
    thread_b.join().unwrap();

    let numbers = vec![100, 46, 34, 10];

    let res = spawn(move || {
        let sum_val = numbers.iter().sum::<usize>();
        sum_val / numbers.len()
    })
    .join()
    .unwrap();

    println!("Main thread is completely executed.... {res}");
}

fn f() {
    let id = current().id();
    println!("This is the Id of f thread {:?}", id);
}



// Notes:
// 1) Here our code spawns two threads and prints their IDs. However, sometimes the IDs don't appear. This happens because 
//    the main thread exits before the spawned threads finish execution.
// 
// 2) By default, closures capture variables by reference (&langs).

