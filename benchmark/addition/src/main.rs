use std::time::Instant;
use rand::Rng;

// Duration of Rust: 7.204 ms
fn main(){
    let start = Instant::now();

    let mut rng = rand::thread_rng();
    let mut result: f32 = 0.0;
    for _ in 0..1000 {
        for _ in 0..1000{
            let num: f32 = rng.gen_range(-1.0..1.0);
            result += num;
        }
    }

    println!("Duration of Rust: {:?}", start.elapsed());
    println!("The result is: {}", result);
}
