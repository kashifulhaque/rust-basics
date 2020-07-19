fn main() {
    let res: i32 = another_function(12, 54);
    println!("Result: {}", res);

    // Expressions
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("x: {}", x);
    println!("y: {}", y);
}

fn another_function(x: i32, y: i32) -> i32 {
    return x * y; // Can also be written as "x * y" only without the "return" keyword and the semi-colon.
}
