fn main() {
    // Passing references to a function
    let str1 = String::from("This is a test of references in Rust");
    let len = calculate_length(&str1);

    println!("\"{}\" has the size: {}", str1, len);

    // Mutable references
    let mut str2 = String::from("This is a mutable string");
    change_string(&mut str2);
    println!("{}", str2);

    // Immutable and Mutable references
    let mut str3 = String::from("An example of Immutable and Mutable reference");

    let ref1 = &str3;
    let ref2 = &str3;
    println!("Reference 1: {}", ref1);
    println!("Reference 2: {}", ref2);

    let ref3 = &mut str3;
    println!("Mutable reference: {}", ref3);

    // Dangling references. Read: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
}

fn calculate_length(str: &String) -> usize {
    str.len()
}

fn change_string(str: &mut String) {
    str.push_str(". Added this line inside \"change_string()\" method.");
}
