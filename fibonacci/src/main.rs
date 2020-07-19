use std::io;

fn main() {
    println!("** PRINT THE FIRST N FIBONACCI nBERS **");
    println!();

    println!("Enter a number: ");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read a line.");
    let n: i32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    print_fibonacci(n);
}

fn print_fibonacci(mut n: i32) {
    let mut x: u32 = 0;
    let mut y: u32 = 1;

    while n >= 0 {
        println!("{}", x);
        let z = x;
        x = x + y;
        y = z;

        n = n - 1;
    }
}
