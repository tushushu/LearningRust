fn main() {
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
    if some_u8_value == Some(3) {
        println!("three");
    }
}
