use std::fs::File;
use std::io::{self, Read, stdin};

fn main() {
    let file_result = read_file();

    match file_result {
        Ok(contents) => println!("{contents}"),
        Err(error) => {
            eprintln!("There was an error: {error:?}");
        }
    }
}

fn read_file() -> Result<String, io::Error> {
    println!("Please enter the name of the file you'd like to read:");

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let mut file_contents = String::new();
    File::open(input.trim())?.read_to_string(&mut file_contents)?;

    Ok(file_contents)
}
/*
❓ What is the ? operator in Rust?
The ? operator is used to simplify error handling when using functions that return Result.

✅ Why Use ??
It helps you avoid writing long match statements. It automatically returns the error if one occurs.

use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("data.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}


Same logic with match (longer):
fn read_file() -> Result<String, io::Error> {
    let mut file = match File::open("data.txt") {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}

| Feature     | Description                              |
| ----------- | ---------------------------------------- |
| What is `?` | Shorthand for error handling in `Result` |
| Use case    | Automatically return error if any        |
| Used in     | Functions returning `Result` or `Option` |
| Benefit     | Cleaner and shorter code                 |


*/
