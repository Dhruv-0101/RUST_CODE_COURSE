fn main() {
    let burger = String::from("Burger");
    add_fries(burger);
    // println!("{burger}");
}

fn add_fries(mut meal: String) {
    meal.push_str(" and Fries");
    println!("{meal}");
}
