static MESSAGE: &str = "Hello, static world!";

pub fn sharing_ownership() {
    println!("The given message si {MESSAGE}");

}


// Learning points 
// 1) When sharing data between two threads where neither thread is guaranteed to outlive the other, 
//    neither of them can be the owner of that data. 

// 2) Any data shared between them will need to live as long as the longest living thread.

// 3) There are several ways to to create Data that’s not owned by a single thread. 
//     Ex:- Statics,Leaking and Reference Counting

