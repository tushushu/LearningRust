use std::string::String;

fn main() {
    example_1();
    example_2();
    example_3();
    example_4();
    example_5();
}

fn example_1() {
    println!("Example 1:");
    let mut s = String::from("Hello, ");
    s.push_str("world!");
    println!("{}\n", s);
}

fn example_2() {
    println!("Example 2:");
    let s1 = String::from("Hello");
    let s2 = s1;
    // Cannot use s1 here!
    println!("{}, world!\n", s2);
}

fn example_3() {
    println!("Example 3:");
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}\n", s1, s2);
}

fn example_4() {
    println!("Example 4:");
    let x = 5;
    let y = x;
    println!("x = {}, y = {}\n", x, y);
}

fn example_5() {
    println!("Example 5:");
    let s1 = gives_ownership();
    let s2 = s1.clone();
    takes_ownership(s1);
    let s3 = takes_and_gives_back(s2);
    println!("{}\n", s3);
}

fn gives_ownership() -> String {
    String::from("Hello world!")
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("{}", a_string);
    a_string
}
