fn main() {
    let penny = Some(Coin::Penny);
    let nickel = Some(Coin::Nickel);
    let dime = Some(Coin::Dime);
    let quarter = Some(Coin::Quarter);
    let other: Option<Coin> = None;

    plus_one(&penny);
    plus_one(&nickel);
    plus_one(&dime);
    plus_one(&quarter);
    plus_one(&other);
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Option<Coin>) -> Option<u8> {
    let value = match coin {
        None => None,
        Some(i) => Some(match i {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }),
    };
    return value;
}

fn plus_one(coin: &Option<Coin>) -> Option<u8> {
    let mut result = value_in_cents(coin);
    result = match result {
        None => None,
        Some(i) => Some(i + 1),
    };
    println!("The {:?} plus one equal to {:?}!", coin, result);
    return result;
}
