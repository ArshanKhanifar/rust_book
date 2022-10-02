#[derive(PartialEq)]
enum Fruit {
    Apple(String),
    Orange,
}

#[derive(PartialEq)]
enum Lights {
    Front,
    Back,
}

#[derive(PartialEq)]
struct SFruit {
    name: String,
}

fn main() {
    let f1 = Fruit::Apple(String::from("Granny Smith"));
    let f2 = Fruit::Apple(String::from("Orchard"));
    let f3 = Fruit::Apple(String::from("Granny Smith"));
    let l1 = Lights::Front;
    let l2 = Lights::Back;

    let sf1 = SFruit {
        name: String::from("Granny Smith"),
    };

    let sf2 = SFruit {
        name: String::from("Granny Smith"),
    };

    if sf1 == sf2 {
        println!("are the same!");
    } else {
        println!("NO!");
    }
}
