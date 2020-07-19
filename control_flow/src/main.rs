fn main() {
    // The "if-else" expression
    let number: i32 = 5;

    if number > 7 {
        println!("Large number");
    } else {
        println!("Small number");
    }

    // The "if-else-else if" expression
    let number: i32 = 8;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using "if" in a "let" expression
    let condition: bool = false;
    let num: i32 = if condition { 5 } else { 10 };

    println!("{}", num);

    // Loops -> There are 3 types of loops. "loop", "while" and "for"
    let mut count: i32 = 0;
    loop {
        println!(
            "{} This code will run indefinitely until count is 500.",
            count
        );
        count = count + 1;

        if count == 500 {
            break;
        }
    }

    // Returning values from a loop
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter = counter + 1;

        if counter == 45 {
            break counter * 6;
        }
    };

    println!("Result: {}", result);

    // While loop
    let mut i: i32 = 16;
    while i >= 0 {
        println!("{}", i);
        i = i - 1;
    }

    // For loop
    let arr: [i32; 10] = [2, 4, 6, 8, 10, 12, 14, 16, 18, 20];
    for a in arr.iter() {
        println!("{}", a);
    }

    for j in 1..10 {
        println!("{}", j);
    }

    for j in (1..10).rev() {
        println!("{}", j);
    }
}
