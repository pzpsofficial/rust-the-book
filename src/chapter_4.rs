fn main() {
    // let s1 = gives_ownership();

    // let s2 = String::from("hello");

    // let s3 = takes_and_gives_back(s2);

    let s4 = String::from("Hello world!");

    let test = first_word(&s4);

    println!("The value of test is {test}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}