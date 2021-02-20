use std::string::String;

fn main() {
    example_1();
    example_2();
    example_3();
    example_4();
}

fn example_1() {
    println!("Example 1:");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}\n", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn example_2() {
    println!("Example 2:");
    let mut s = String::from("Hello ");
    change(&mut s);
    println!("{}\n", s);
}

fn change(s: &mut String) {
    s.push_str("world!");
}

fn example_3() {
    println!("Example 3:");
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let mut r3 = s.clone(); // Cannot use let r3 = &mut s here.
    r3.push_str(" world!");
    println!("{}, {}, and {}\n", r1, r2, r3);
}

fn example_4() {
    println!("Example 4:");
    let r = {
        let s = String::from("hello");
        s // cannot use &s here.
    };
    println!("r is {}\n", r);
}
