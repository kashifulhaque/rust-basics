fn main() {
    let mut s = String::from("Hello"); // The memory will be allocated on the heap.
    s.push_str(", heap!");

    println!("{}", s);

    // The following code creates a "deep copy" -> It copies the pointer which is stored on the stack and the data it refers to on the heap.
    let s1 = String::from("A string to be cloned!");
    let s2 = s1.clone();

    println!("String1: {}", s1);
    println!("String2: {}", s2);
}
