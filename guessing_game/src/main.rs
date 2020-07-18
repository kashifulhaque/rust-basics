use std::io;

fn main() {
    println!("**** ENTER SOMETHING AND WE'LL SPIT IT BACK ON THE CONSOLE ****"); // This is a macro, not a function. That bang "!" sign denotes a macro.

    println!("Enter something: ");
    let mut guess = String::new(); // That "mut" keyword marks a variable as mutable. In Rust, variables are immutable by default.

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to take input.");

    println!("Your said: {}", guess);
}
