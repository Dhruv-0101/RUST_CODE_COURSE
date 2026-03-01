fn main() {
    let grams_of_protein = "100.345";
    let grams_of_protein = 100.345; //variable shadowing
    let mut grams_of_protein = 100; //variable shadowing
    grams_of_protein = 105;

    println!("grams of protein: {grams_of_protein}");
}
