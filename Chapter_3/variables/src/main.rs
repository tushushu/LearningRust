fn main() {
    // Mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // Shadowing
    let x = x + 1;
    println!("The value of x is: {}", x);
}
