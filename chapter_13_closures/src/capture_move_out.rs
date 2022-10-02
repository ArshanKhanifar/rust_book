fn returns_optional_string() -> Option<String> {
    return Option::Some("Hello".to_string());
}

pub fn unwraps_optional_string() {
    let something = returns_optional_string();
    let unwrapped = something.unwrap_or_else(|| "error".to_string());

    // doing this twice  will throw an error!
    //let unwrapped_again = something.unwrap_or_else(|| "somebadshit".to_string());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/// # here's a heading
/// here is an example of a documentation comment
/// 1. I
/// 2. Can use
/// 3. Numbered lists
///
/// Orrrr....
///
/// I _can_ **use** __some__ unnumbered list:
/// * like
/// * this
///
/// code blocks?
/// ```
/// println!("hello, world!");
/// ```
///

pub fn sort_rect_sort_by_key_is_fn_mut() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // sort_by_key is FnMut because it DOESNT move the captured value out of its body, but
    // it might mutate the captured values. So it can be called more than once.
    // this is rust's example but it's boring cuz it doesn't mutate the rectangle!
    list.sort_by_key(|r| r.width);
    // NOTE: that in this closure there's NO captured value!
    let some_const = 5;
    list.sort_by_key(move |r| r.width * some_const);
    println!("{:#?}", list);
    println!("some const: {some_const}");
}
