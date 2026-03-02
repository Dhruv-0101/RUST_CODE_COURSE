fn main() {
    println!("Some status update");
    println!("Some status update 2");

    eprintln!("Some error message");
    eprintln!("Some error message 2");

    // cargo run --bin 3_standard_error > out.txt 2> err.txt
}
