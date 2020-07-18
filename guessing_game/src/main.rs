use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("**** ENTER SOMETHING AND WE'LL SPIT IT BACK ON THE CONSOLE ****"); // This is a macro, not a function. That bang "!" sign denotes a macro.

    // Generate a secret random number
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number: {}", secret_number);

    println!("Enter something: ");
    let mut guess = String::new(); // That "mut" keyword marks a variable as mutable. In Rust, variables are immutable by default.

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to take input.");

    let guess: u32 = guess.trim().parse().expect("Enter a number!");

    println!("Your said: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Your guess is small."),
        Ordering::Greater => println!("Your guess is big."),
        Ordering::Equal => println!("Your guess is the correct one."),
    }
}
