fn main() {
    let pizza_diameters: Vec<i32> = vec![8, 10, 12, 14];
    println!("{pizza_diameters:?}");

    let pastas: Vec<&str> = vec!["Rigatoni", "Angel hair", "Fettucine"];
    println!("{pastas:?}");
}
/*
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(10);
    numbers.push(20);
    println!("{:?}", numbers); // Output: [10, 20]

    let mut names = vec![]; // Type will be inferred if used later
    names.push("Alice");
    names.push("Bob");
    println!("{:?}", names); // Output: ["Alice", "Bob"]
*/
