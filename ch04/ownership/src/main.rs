fn main() {
    let s = String::from("hello");          // sがスコープに入る

    takes_ownership(s);                     // sの値が関数にムーブ、ここではもう有効ではない。

    // println!("{}!!!!", s)                // sはdropされているので、エラーになる

    let x = 5;                              // xがスコープに入る

    makes_copy(x);                          // xも関数にムーブされるが、i32がCopyなので、問題なし

    println!("{}!!!!!!", x);                // エラーにならない

    let s1 = gives_ownership();            // gives_ownershipは戻り値をs1にムーブする

    let s2 = String::from("hello");         // s2がスコープに入る

    let _s3 = takes_and_gives_back(s2);     // s2はtakes_and_gives_backにムーブされ、戻り値もｓ3にむーぶされる

    let (s4, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s4, len);

}                                           // s3, s1はここでドロップされる。ｓ2はムーズ済みなの何も起きない

fn takes_ownership(some_string: String) {   // some_stringがスコープに入る
    println!("{}", some_string);
}                                           // some_stringがスコープを抜け、dropが呼ばれ、メモリが開放される

fn makes_copy(some_integer: i32) {          // some_integerがスコープに這入る
    println!("{}", some_integer);
}                                           // some_integerがスコープを抜け、なにもない。Copyなので。

fn gives_ownership() -> String {
    let some_string = String::from("hello");    // some_stringがスコープに入る

    some_string                             // some_stringが返され、呼び出し元関数にムーブされる
}

fn takes_and_gives_back(a_string: String) -> String {   // a_stringがスコープに入る
    a_string                                // a_stringが返され呼び出し元関数にムーブされる
}

// return tuple
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}