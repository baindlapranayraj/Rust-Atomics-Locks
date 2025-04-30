pub mod topics;

#[allow(unused_imports)]
use topics::{
    safty_threads::safty_threads, scopedthreads::scoped_threads, shared_threads::sharing_ownership,
    threads::main_thread,
};

fn main() {
    // main_thread();
    // scoped_threads();
    // sharing_ownership();

    safty_threads();

    println!("Hello world from Neovim and Rust");
}
