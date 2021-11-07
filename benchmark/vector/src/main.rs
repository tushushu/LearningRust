use rand::Rng;
use std::time::Instant;

// Duration of Rust: 10.045 ms
fn main() {
    let mut rng = rand::thread_rng();
    let n: i32 = 10000000;
    let mut vec: Vec<f32> = Vec::new();
    for _ in 0..n {
        let num: f32 = rng.gen_range(-1.0..1.0);
        vec.push(num);
    }

    let start = Instant::now();
    let result: f32 = vec.iter().sum();


    println!("Duration of Rust: {:?}", start.elapsed());
    println!("The result is: {}", result);
}
