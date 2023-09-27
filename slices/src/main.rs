fn main() {
    let mut s = String::from("Hello world!");

    let word = first_word(&s);
    println!("{}", word);

    s.clear(); // Stringを空にする。つまり、""と等しくする

    // String Slice
    let s = String::from("hello world");

    let _hello = &s[0..5];
    let _world = &s[6..11];

    let _slice = &s[0..2];
    let _slice = &s[..2];

    let len = s.len();
    let _slice = &s[3..len];
    let _slice = &s[3..];

    let _slice = &s[0..len];
    let _slice = &s[..];

    let s = String::from("hello world");

    let word = first_word_2(&s);
    // s.clear();

    println!("the first word is: {}", word);

    let my_string = String::from("hello world");
    let word = first_word_3(&my_string[..]);
    println!("{}", word);

    let my_string_literal = "hello world";

    let word = first_word_3(&my_string_literal[..]);
    println!("{}", word);
    let word = first_word_3(my_string_literal);
    println!("{}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
