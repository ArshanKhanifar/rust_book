enum List {
    Cons(i32, Box<List>),
    /*
    this would fail because  rust can't figure out how big the variable is
     */
    //Cons(i32, List),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
