fn main() {
    // Constant
    const MAX_LIMIT: u32 = 100_000;

    // Mutable variables
    let mut x = 10;
    println!("x = {}", x);
    x = 15;
    println!("x = {}", x);

    // Shadowing variables
    let y = 10;

    let y = y + 20;

    let y = y + 50;
    println!("y = {}", y);
    /**************************************************/
    let spaces = "       ";
    let spaces = spaces.len();

    println!("No. of spaces: {}", spaces);
}
