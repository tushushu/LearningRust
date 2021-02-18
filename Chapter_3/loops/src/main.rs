fn main() {
    example_1(10);
    example_2(3);
    example_3();
    example_4();
}

fn example_1(max_count: i32) {
    println!("Example 1:");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == max_count {
            break counter * 2;
        }
    };

    println!("The result is {}\n", result);
}

fn example_2(start: i32) {
    println!("Example 2:");
    let mut number = start;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!\n");
}

fn example_3() {
    println!("Example 3:");
    let arr = [10, 20, 30, 40, 50];

    for element in arr.iter() {
        println!("the value is: {}", element);
    }
    println!();
}

fn example_4() {
    println!("Example 4:");
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!();
}
