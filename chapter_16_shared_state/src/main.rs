use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn simple() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

//fn multiple_threads_not_multiple_owned_throws_error() {
//    // this will throw error cuz counter gets moved into the threads...
//    let counter = Mutex::new(0);
//    let mut handles = vec![];
//
//    for _ in 0..10 {
//        let handle = thread::spawn(move || {
//            let mut num = counter.lock().unwrap();
//
//            *num += 1;
//        });
//        handles.push(handle);
//    }
//
//    for handle in handles {
//        handle.join().unwrap();
//    }
//
//    println!("Result: {}", *counter.lock().unwrap());
//}

// commented out because the compile time detection
//fn multiple_threads_using_rc() {
//    // this will throw error too because Rc is not thread safe (clone might get interrupted, and
//    // you'll end up with the wrong count)
//    let counter = Rc::new(Mutex::new(0));
//    let mut handles = vec![];
//
//    for _ in 0..10 {
//        let counter = Rc::clone(&counter);
//        let handle = thread::spawn(move || {
//            let mut num = counter.lock().unwrap();
//
//            *num += 1;
//        });
//        handles.push(handle);
//    }
//
//    for handle in handles {
//        handle.join().unwrap();
//    }
//
//    println!("Result: {}", *counter.lock().unwrap());
//}

fn multiple_threads_using_arc() {
    // arc stands for atomic reference counter:
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn main() {
    multiple_threads_using_arc();
}
