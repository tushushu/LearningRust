use std::cmp::PartialOrd;

fn main() {
    let a = 3;
    let b = 2;
    let c = max_num(a, b);
    println!("Max number is {}", c);

    let a = 3.14;
    let b = 2.14;
    let c = max_num(a, b);
    println!("Max number is {}", c);
}

fn max_num<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        return b;
    }
    return a;
}
