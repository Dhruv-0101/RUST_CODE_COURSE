fn main() {
    let mut car = String::from("Red");
    let ref1 = &mut car;//mutable reference(isko kal tune khali kar diya to? yani ki heap me tumne car ko khali kar diya)
    let ref2 = &mut car;
    println!("{ref2}");//yeh reference valid nahi hai bhai
    // println!("{ref1}");//uska reference valid hai bhai
}
