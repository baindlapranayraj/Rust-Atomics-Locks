use std::{sync::Mutex, thread};

// ++++++++++++++++++++++++  sharing the mutable data between multiple threads +++++++++++++++++++++++++++++++++++

pub fn safty_threads() {
    let name = Mutex::new(String::from("Pranay Raj"));

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut lock = name.lock().unwrap();

                let vecc = vec!["👑", "🦄", "🔥", "🦀"];

                for emoji in vecc {
                    lock.push_str(emoji);
                }
            });
        }
    });

    println!("The name : {:?}", name);
}
