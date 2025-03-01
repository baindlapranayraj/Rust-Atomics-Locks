use std::thread;

static MESSAGE: &str = "Hello, static world!";

pub fn sharing_ownership() {
    println!("The given message is {MESSAGE}");
    thread::spawn(|| dbg!(&MESSAGE));
    thread::spawn(|| dbg!(&MESSAGE));

    println!("Hello Rust in neovim 🦀");
}

// Learning points:
// 1) When sharing data between two threads where neither thread is guaranteed to outlive the other,
//    neither of them can be the owner of that data.
//
// 2) Any data shared between them will need to live as long as the longest living thread.
//
// 3) There are several ways to create Data that's not owned by a single thread.
//    Ex:- Statics, Leaking and Reference Counting

// 4) A static item has a constant initializer, is never dropped, and already exists before the main function of the 
//    program even starts. Every thread can borrow it, since it’s guaranteed to always exist.
