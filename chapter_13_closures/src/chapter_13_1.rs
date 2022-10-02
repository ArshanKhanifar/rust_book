use std::thread;

pub fn mutable_closure() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    // won't error
    borrows_mutably();
    // if we don't have any more calls past this point, the mutable borrow ends right here...
    // Rust's Documentation EXPLICITLY SAYS THIS:
    // "We don’t use the closure again after the closure is called, so the mutable borrow ends."
    // https://doc.rust-lang.org/book/ch13-01-closures.html

    println!("After calling closure: {:?}", list);
    // WILL error (the mutable borrow ends after the last call to the closuree)
    // borrows_mutably();
}

pub fn using_closures_with_forced_ownership() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    /*
    closures by default don't take ownership of the parameters passed into them,
    so you gotta explicitly move the ownership into the closure with the keyword move
    if move is omitted here, then you'll get an error.
    */
    //thread::spawn(|| println!("From thread: {:?}", list))
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    /*
    you can't use the variable after. Is there a way to pull it out? Idk...
     */
    //println!("After spawning the thread: {:?}", list);
}

pub fn using_thread_with_normal_fn_so_no_move_required() {
    let list = vec![1, 2, 3];
    println!("list before: {:?}", list);

    /*
    TURNS OUT YOU CAN'T DO THIS
    so you gotta explicitly move the ownership into the closure with the keyword move
    if move is omitted here, then you'll get an error.
    */
    //thread::spawn(|| println!("From thread: {:?}", list))

    fn thread_body_doing_its_own_thing() {
        let a = 123;
        println!("here is: {a}");
    }

    // WILL ERROR AT COMPILE TIME
    //fn thread_body_using_variable_from_parent_context() {
    //    println!("From thread: {:?}", list);
    //}
    //thread::spawn(thread_body_using_variable_from_parent_context).join().unwrap();
    thread::spawn(thread_body_doing_its_own_thing)
        .join()
        .unwrap();
}
