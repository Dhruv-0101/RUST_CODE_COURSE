fn main() {
    // &str
    // String

    let pirate = "Bloodhook";

    let sailer = String::from(pirate);

    let bad_guy = sailer.to_string();

    println!("{pirate} and {sailer} and {bad_guy}");

    // let first_initial = &pirate[0];//can not access characters by index.access characters by slice only.although one character is equal to one byte but this is not always true like in case of utf-8 like in case of emoji
    // let first_initial = &sailer[0]
    let first_initial = &pirate[0..4];

    println!("{first_initial}");
}
