fn main() {
    let _guess: u32 = "42".parse().expect("Not a number!");

    // Float
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Number
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("The value of sum is: {}", sum);
    println!("The value of difference is: {}", difference);
    println!("The value of product is: {}", product);
    println!("The value of quotient is: {}", quotient);
    println!("The value of remainder is: {}", remainder);

    // Bool
    let _t = true;
    let _f: bool = false;

    // Char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of char is: {}", c);
    println!("The value of char is: {}", z);
    println!("The value of char is: {}", heart_eyed_cat);

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of tup.0 is: {}", five_hundred);
    println!("The value of tup.1 is: {}", six_point_four);
    println!("The value of tup.2 is: {}", one);

    // Array
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("The value of array first is: {}", first);
    println!("The value of array second is: {}", second);

    // Can not access this index
    // let index = 10;
    // let element = a[index];
    // println!("The value of element is: {}", element);
}
