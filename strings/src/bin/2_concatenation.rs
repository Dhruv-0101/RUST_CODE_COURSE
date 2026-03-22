fn main() {
    let first_name = String::from("Sylvester");
    let last_name = String::from("Stallone");

    let full_name = first_name + &last_name;
    println!("{full_name}");

    //  fn add(mut self, other: &str) -> String {
    //     self.push_str(other);
    //     self
    // }

    // Invalid
    // println!("{first_name}");
    // valid
    println!("{last_name}");
}
