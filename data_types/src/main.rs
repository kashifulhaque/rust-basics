fn main() {
    // Parsing a string to an unsigned 32-bit integer
    let guess: u32 = "42".parse().expect("That's not a number");
    println!("Guess: {}", guess);

    // Floating-point types
    let x = 3.14; // f64: By default, this will be a 64-bit floating point number
    let y: f32 = 4.201; // f32: We can explicitly tell it to be a 32-bit floating point number

    // Boolean types
    let t = true;
    let f: bool = false; // Explicit type annotation

    // Char types -> 4 bytes in size
    let small_char = 'z';
    let capital_char = 'Z';
    let emoji = '👌';

    /* Arithmetic Operations */
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    /* Compound data types */
    // Tuples
    let tupl: (i32, f64, u8) = (400, 54.76, 1); // Type annotations are optional
    let (x, y, z) = tupl; // Destructure a tuple
    println!("float value of tuple: {}", y);
    let first_element = tupl.0;
    let second_element = tupl.1;
    let third_element = tupl.2;

    // Arrays
    let arr = [2, 4, 6, 8, 10];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let its_all_three = [3; 5];
}
