fn main() {
    let s = String::from("hello world");
    let first = first_word(&s);
    let second = second_word(&s);
    println!(
        "Ths first word of '{}' is: {} and second is: {}",
        &s, first, second
    );

    let _hello = &s[0..5];
    let _world = &s[6..11];
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

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[(i + 1)..];
        }
    }

    &s[..]
}
