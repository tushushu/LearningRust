fn main() {
    example_1(7);
    example_2(5, 6);
}

fn example_1(number: i32) {
    println!("Example 1:");
    if number % 5 == 0 {
        println!("number is divisible by 5");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number {} is not divisible by 5, 3 or 2\n", number);
    }
}

fn example_2(x: i32, y: i32) {
    println!("Example 2:");
    let condition = true;
    let number = if condition { x } else { y };
    println!("number is {}\n", number);
}
