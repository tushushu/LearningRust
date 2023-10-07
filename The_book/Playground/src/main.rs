fn main() {
    println!("Hello, world!");
}

struct MyVec<'a, T> {
    value: &'a [T],
    length: usize,
}

trait Len<T> {
    fn sum_my_vec(&self) -> T;
}

impl<T> SumMyVec<T> for MyVec<T> {
    fn sum_my_vec(&self) -> T {
        return self.value.sum();
    }
}
