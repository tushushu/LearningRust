fn main() {
    let z = another_function(5, 6);
    println!("The value of x + y is {}", z);

    let w = {
        let w = 2;
        w + 1
    };
    println!("The value of w is {}", w);
}

fn another_function(x: i32, y: i32) -> i32{
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    x + y
}
