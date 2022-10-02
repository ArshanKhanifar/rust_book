fn indexing_into_strings() {
    let hello = String::from("Hola");
    let hello = "Здравствуйте";
    let answer = &hello[..3];
    println!("andwer: {answer}");
}

fn string_stuff() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    //let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    let s = format!("{}-{}", s1, s2);
    println!("s: {s}");
    println!("s1 is still: {s1}");
}

fn captures(value: String) {
    // does nothing
    println!("{value}");
}

fn borrowed_twice() {
    let a = "hello".to_string();
    let suba = &a[..3];
}

fn something() {
    let v = vec![1, 2, 3, 5];
    let third = v[2];
    let sixth = v[5];
    println!("sixth element is: {}", sixth);
    //match third {
    //    Some(third) => println!("the third element is: {}", third),
    //    None => println!("There is no third element"),
    //}
}
fn basic() {
    let mut v = Vec::new();
    v.push("hello");
    v.push("hello");
    v.push("hello");
    v.push("hello");
    v.push("hello");
    v.push("hello");
    let b = vec![1, 2, 35];
    println!("this is our vec: {:?}", v);
    println!("this is our vec: {:?}", b);
}

fn main() {
    indexing_into_strings();
}
