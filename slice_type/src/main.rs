fn main() {
    let s = String::from("Hi there, donutz");
    let word = get_first_word(&s);

    println!("Word size: {}", word);

    // String slices
    let str = String::from("Slice this");
    let slice = &str[0..5];
    let this = &str[6..10];

    println!("Slice: {}", slice);
    println!("This: {}", this);
}

fn find_first_word(str: &String) -> usize {
    let bytes = str.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return i;
        }
    }

    str.len()
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
