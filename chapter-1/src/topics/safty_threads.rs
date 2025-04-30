use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

// ============================= Sharing Mutable Data Between Multiple Threads =================================

pub fn safety_threads() {
    // 1) Using Arc (for sharing ownership across threads) and Mutex (for mutating the data safely)
    let name = Arc::new(Mutex::new(String::from("Pranay Raj")));

    // Using a scoped thread environment for safe thread handling (Rust 1.63+)
    thread::scope(|s| {
        for _ in 0..10 {
            // Each thread gets a clone of the Arc pointer
            let name = Arc::clone(&name);
            s.spawn(move || {
                // Lock the mutex to get mutable access to the string
                let mut lock = name.lock().unwrap();

                // A vector of emojis to append
                let vecc = vec!["👑", "🦄", "🔥", "🦀"];

                // Append each emoji to the string
                for emoji in vecc {
                    lock.push_str(emoji);
                }
                // Mutex is automatically unlocked when 'lock' goes out of scope
            });
        }
    });

    // 2) Using mpsc (multiple producer, single consumer) channels for sharing data between threads

    // Create a channel with a transmitter (tx) and receiver (rx)
    let (tx, rx) = mpsc::channel();

    // Clone the transmitter so multiple threads can send messages
    let tx1 = tx.clone();

    // First thread sends a series of messages with a delay
    thread::spawn(move || {
        let values = vec![
            String::from("Hello from spawn thread"),
            String::from("Hello from second vec"),
            String::from("Bhudha"),
        ];

        for value in values {
            tx1.send(value).unwrap(); // Send the value to the receiver
            thread::sleep(Duration::from_millis(1000)); // Sleep for 1 second
        }
    });

    // Second thread sends another series of messages with a different delay
    thread::spawn(move || {
        let values = vec![
            String::from("🔥 from spawn thread"),
            String::from("🔥 from second vec"),
            String::from("🔥 Bhudha"),
        ];

        for value in values {
            tx.send(value).unwrap(); // Send the value to the receiver
            thread::sleep(Duration::from_millis(500)); // Sleep for 0.5 seconds
        }
    });

    // The main thread receives and prints messages as they arrive
    for received in rx.iter() {
        println!("The receiver got: {received}");
    }
}

// +++++++++++++++ Learnings +++++++++++++++
// 1) mpsc channels are like river streams: when you place a duck 🦆 (data) on the upstream (tx),
//    it goes downstream (rx).
//
// 2) There can be multiple upstreams (producers) for one downstream (consumer), but only one receiver.
//
// 3) Arc + Mutex is used for shared mutable state; channels are used for message(the message is data) passing between threads.
