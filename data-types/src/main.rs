fn main() {
    let _x = 2.0; // f64
    let _y: f32 = 3.0; //f32

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0

    // remainder
    let _remainder = 43 % 5;

    // boolean
    let _t = true;

    let _f: bool = false; // with explicit type annotation 明示的型注釈付き

    // char
    let _c = 'z';
    let _z = 'Z';
    let _heart_eyed_cat = '😻';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    // List スタック上に確保される一塊のメモリ
    let _a = [1, 2, 3, 4, 5];
    let _b: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // [3, 3, 3, 3, 3]
    let _first = a[0];
    let _second = a[1];
}
