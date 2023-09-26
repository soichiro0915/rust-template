fn main() {
    let _s = String::from("hello");

    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); error
    println!("{}, world!", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello"); // sがスコープに入る

    takes_ownership(s); // sの値が関数にムーブされ...
                        // ... ここではもう有効ではない

    let x = 5; // xがスコープに入る

    makes_copy(x); // xも関数にムーブされるが、
                   // i32はCopyなので、この後にxを使っても
                   // 大丈夫

    let _s1 = gives_ownership(); // gives_ownershipは、戻り値をs1に
                                 // ムーブする

    let _s2 = String::from("hello"); // s2がスコープに入る

    let _s3 = takes_and_gives_back(s2); // s2はtakes_and_gives_backにムーブされ
                                        // 戻り値もs3にムーブされる

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    //'{}'の長さは、{}です
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string // some_stringが返され、呼び出し元関数に
                // ムーブされる
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_stringが返され、呼び出し元関数にムーブされる
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返します

    (s, length)
}
