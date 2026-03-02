fn main() {
    let a = 1;
    panic!("Something went wrong!");
}
/*
re - are  errors that we can define code to handle
ure - are errors that we can't handle(program can be unable to proceed) ex.panic

✅ Recoverable Error Example (Result):
Recoverable errors are expected and can be handled using Result.

use std::fs::File;

fn main() {
    let file_result = File::open("some_file.txt");

    match file_result {
        Ok(file) => println!("File opened successfully!"),
        Err(e) => println!("Failed to open file: {}", e), // recoverable
    }
}
Explanation: File might not exist — that's expected.

We handle the error using match, so it's recoverable.

-------------------------------------------------------------------------------

❌ Unrecoverable Error Example (panic!):
Unrecoverable errors cause the program to stop immediately.

fn main() {
    let numbers = [1, 2, 3];
    println!("{}", numbers[5]); // panic!
}

OR explicitly:
fn main() {
    panic!("Something went terribly wrong!");
}
Explanation: Accessing out-of-bounds or calling panic!() crashes the program.

These are unrecoverable.
*/
