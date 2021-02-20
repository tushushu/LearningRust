use std::string::String;

fn main() {
    let s1 = String::from("hello world");
    let word_1 = first_word(&s1[..]);
    println!("The first word is {}\n", word_1);

    let s2 = "good morning";
    let word_2 = first_word(&s2);
    println!("The first word is {}\n", word_2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
