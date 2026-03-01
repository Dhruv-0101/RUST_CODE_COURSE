fn main() {
    let registrations = (true, false, true);
    let first = registrations.0;
    println!("{first} and {registrations:?}");

    let languages = (String::from("Rust"), String::from("JavaScript"));
    let first = &languages.0;
    println!("{first} and {languages:?}");
}
/*
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
} */
