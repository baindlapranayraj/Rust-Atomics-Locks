pub mod topics;

#[allow(unused_imports)]
use topics::{
    scopedthreads::scoped_threads, 
    threads::main_thread,
    shared_threads::sharing_ownership
};

fn main() {
    // main_thread();
    // scoped_threads();
    sharing_ownership();
}

