fn main() {
    // Addition
    let nan = f32::NAN;
    let num = 1.0;
    // nan
    println!("nan + 1.0 is {}", nan + 1.0);


    // Cmp

    // false
    let result = nan > num;
    println!("NAN > 1.0 is {}", result);
    // false
    let result = nan < num;
    println!("NAN < 1.0 is {}", result);
    // false
    let result = nan == num;
    println!("NAN == 1.0 is {}", result);
    // true
    let result = nan != num;
    println!("NAN != 1.0 is {}", result);

    // Partial cmp, see https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#tymethod.partial_cmp
    // None
    println!("1.0.partial_cmp(nan) is {:?}", 1.0.partial_cmp(&nan));
    // Less
    println!("1.0.partial_cmp(2.0) is {:?}", 1.0.partial_cmp(&2.0).unwrap());
    // Greater
    println!("1.0.partial_cmp(0.0) is {:?}", 1.0.partial_cmp(&0.0).unwrap());
    // Equal
    println!("1.0.partial_cmp(1.0) is {:?}", 1.0.partial_cmp(&1.0).unwrap());

    
    // Sort with nan will panic.
    let mut vec = [1.0, nan, 2.0];
    vec.sort_by(|x, y| x.partial_cmp(y).unwrap());
    println!("sorted [1.0, nan, 2.0] is {:?}", vec);

}
