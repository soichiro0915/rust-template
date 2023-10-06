// #![allow(unused)]
// fn main() {
//     let some_u8_value = Some(3u8);
//     match some_u8_value {
//         Some(3) => println!("three"),
//         _ => (),
//     }

//     if let Some(3) = some_u8_value {
//         println!("three");
//     }
// }

#![allow(unused)]
fn main() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    match coin {
        // {:?}州のクォーターコイン
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // let mut count = 0;
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     count += 1;
    // }
}
