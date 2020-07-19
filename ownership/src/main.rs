/**
 * Rust Programming: What is Ownership? (Chapter 4, Lesson 1 of the Rust Programming Language)
 * Read more: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
 */
fn main() {
    let mut s = String::from("Hello"); // The memory will be allocated on the heap.
    s.push_str(", heap!");

    println!("{}", s);

    // The following code creates a "deep copy" -> It copies the pointer which is stored on the stack and the data it refers to on the heap.
    let s1 = String::from("A string to be cloned!");
    let s2 = s1.clone();

    println!("String1: {}", s1);
    println!("String2: {}", s2);

    // Ownership and Functions
    let str = String::from("こんにちは"); // "str" comes into scope
    takes_ownership(str); // The value of "str" is moved to the function and it is no longer valid here

    let x = 10; // "x" comes into scope
    makes_copy(x); // As "i32" is Copy, it is perfectly fine to use "x" afterwards before it goes out of scope

    // Returning values and scope
    let str1 = gives_ownership();
    let str2 = String::from("お元気ですか");
    let str3 = takes_and_gives_back(str2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // "some_string" goes out of scope here and "drop" is called. The memory is also freed

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
} // "some_int" goes out of scope here

fn gives_ownership() -> String {
    let some_string = String::from("Hi.");
    some_string // Can also be written as "return some_string;"
}

fn takes_and_gives_back(text: String) -> String {
    return text;
}
