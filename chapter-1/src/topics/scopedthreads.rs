use std::thread;

pub fn scoped_threads() {
    let mut num = vec![10, 2, 12, 55];

    thread::scope(|s| {  // 1
        s.spawn(|| { // 2
            println!("First spawn thread from scope function we are accesing {:?}",num);
        });

        s.spawn(|| {  // 2
            let sum = num.iter().sum::<usize>();
            let res = sum / num.len();
            println!("The second thread of this scoope function is {res}");
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