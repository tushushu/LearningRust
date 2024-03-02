fn main() {
    // // 0. Uninitialized reference
    // let x: &i32;
    // println!("{}", x);

    // // 1. Assigned by uninitialized reference
    // let z = 100;
    // let x: &i32;
    // let mut y = &z;
    // y = x;
    // println!("{}", y);

    // // 2. Using Option<i32> type
    // let mut x: Option<i32>;

    // x = Some(100);
    // println!("{:?}", x);

    // x = None;
    // println!("{:?}", x);

    // x += 3;
    // println!("{:?}", x);

    // let mut y = match x {
    //     None => panic!("x is None"),
    //     Some(k) => k,
    // };

    // y += 3;
    // println!("{}", y)

    // 3. Using Option<&Student> type
    // In this case there are 6 types in Rust:
    // - Student
    // - &Student
    // - Option<Student>
    // - &Option<Student>
    // - Option<&Student>
    // - &Option<&Student>
    struct Student {
        age: i32,
    }
    let s: Option<Student> = None;
    println!("{}", s.age);
}
