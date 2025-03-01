pub mod topics;

#[allow(unused_imports)]
use topics::{
    scopedthreads::scoped_threads, shared_threads::sharing_ownership, threads::main_thread,
};

fn main() {
    main_thread();
    scoped_threads();
    sharing_ownership();

    println!("Hello world from Neovim and Rust");
}
