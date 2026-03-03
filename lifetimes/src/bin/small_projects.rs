//------------------------------------------------------------------------------------testing & starting-------------------------------------------------------------------------------------//
// fn main() {
//     println!("Hello, world!");
// }
//------------------------------------------------------------------------------------testing & starting-------------------------------------------------------------------------------------//

//------------------------------------------------------------------------------------calculator-basic-------------------------------------------------------------------------------------//
// use std::io;

// //Calculator - add, sub, mul, div, square root
// fn add(a: u32, b: u32) -> u32 {
//     a + b
// }
// fn sub(a: u32, b: u32) -> u32 {
//     if a > b {
//         return a - b;
//     }
//     return b - a;
// }

// fn div(a: u32, b: u32) -> f64 {
//     (a as f64) / (b as f64)
// }

// fn mul(a: u32, b: u32) -> u32 {
//     a * b
// }

// fn square_root(a: u32) -> f64 {
//     (a as f64).sqrt()
// }

// fn main() {
//     let mut input_one = String::new();
//     println!("Enter your first number");
//     io::stdin()
//         .read_line(&mut input_one)
//         .expect("Not valid input");

//     let num_one: u32 = input_one.trim().parse().expect("Not a valid number");

//     let mut input_two = String::new();
//     println!("Enter your second number");
//     io::stdin()
//         .read_line(&mut input_two)
//         .expect("Not valid input");

//     let num_two: u32 = input_two.trim().parse().expect("Not a valid number");

//     println!("The addition result is {}", add(num_one, num_two));
//     println!("The subtraction result is {}", sub(num_one, num_two));
//     println!("The multiplication result is {}", mul(num_one, num_two));
//     println!("The division result is {}", div(num_one, num_two));
//     println!("The square root of num_one is {}", square_root(num_one));
//     println!("The square root of num_two is {}", square_root(num_two));
// }
//------------------------------------------------------------------------------------calculator-basic-------------------------------------------------------------------------------------//

//------------------------------------------------------------------------------------calculator-infinity-solve-------------------------------------------------------------------------------------//

// use std::io;

// //Calculator - add, sub, mul, div, square root
// fn add(a: u32, b: u32) -> u32 {
//     a + b
// }
// fn sub(a: u32, b: u32) -> u32 {
//     if a > b {
//         return a - b;
//     }
//     return b - a;
// }

// fn div(a: u32, b: u32) -> Option<f64> {
//     if b == 0 {
//         return None;
//     }
//     return Some((a as f64) / (b as f64));
// }

// fn mul(a: u32, b: u32) -> u32 {
//     a * b
// }

// fn square_root(a: u32) -> f64 {
//     (a as f64).sqrt()
// }

// fn main() {
//     let mut input_one = String::new();
//     println!("Enter your first number");
//     io::stdin().read_line(&mut input_one).expect("Not valid input");

//     let num_one: u32 = input_one.trim().parse().expect("Not a valid number");

//     let mut input_two = String::new();
//     println!("Enter your second number");
//     io::stdin().read_line(&mut input_two).expect("Not valid input");

//     let num_two: u32 = input_two.trim().parse().expect("Not a valid number");

//     println!("The addition result is {}", add(num_one, num_two));
//     println!("The subtraction result is {}", sub(num_one, num_two));
//     println!("The multiplication result is {}", mul(num_one, num_two));
//     // println!("The division result is {}", div(num_one, num_two));
//     match div(num_one, num_two) {
//         Some(result) => println!("The division result is {}", result),
//         None => println!("Cannot divide by zero"),
//     }
//     println!("The square root of num_one is {}", square_root(num_one));
//     println!("The square root of num_two is {}", square_root(num_two));
// }
//------------------------------------------------------------------------------------calculator-infinity-solve-------------------------------------------------------------------------------------//

//------------------------------------------------------------------------------------calculator-remove-redudant-code-------------------------------------------------------------------------------------//
// use std::io;

// // Calculator - add, sub, mul, div, square root
// fn add(a: u32, b: u32) -> u32 {
//     a + b
// }

// fn sub(a: u32, b: u32) -> u32 {
//     if a > b {
//         return a - b;
//     }
//     return b - a;
// }

// fn div(a: u32, b: u32) -> Option<f64> {
//     if b == 0 {
//         return None;
//     }
//     return Some((a as f64) / (b as f64));
// }

// fn mul(a: u32, b: u32) -> u32 {
//     a * b
// }

// fn square_root(a: u32) -> f64 {
//     (a as f64).sqrt()
// }

// fn get_number(prompt: &str) -> u32 {
//     loop {
//         let mut input = String::new();
//         println!("{}", prompt);
//         io::stdin().read_line(&mut input).expect("Not valid input");
//         match input.trim().parse::<u32>() {
//             Ok(num) => {
//                 return num;
//             }
//             Err(_) => println!("Not a valid number"),
//         }
//     }
// }

// fn main() {
//     let num_one = get_number("Enter your first number");
//     let num_two = get_number("Enter your second number");

//     println!("The addition result is {}", add(num_one, num_two));
//     println!("The subtraction result is {}", sub(num_one, num_two));
//     println!("The multiplication result is {}", mul(num_one, num_two));

//     match div(num_one, num_two) {
//         Some(result) => println!("The division result is {}", result),
//         None => println!("Cannot divide by zero"),
//     }

//     println!("The square root of num_one is {}", square_root(num_one));
//     println!("The square root of num_two is {}", square_root(num_two));
// }
//------------------------------------------------------------------------------------calculator-remove-redudant-code-------------------------------------------------------------------------------------//

//------------------------------------------------------------------------------------rendom-number-------------------------------------------------------------------------------------//
// User will guess a number
// random functio which will generate a random value
// guessed number<random value -> gn is less than rv
// guessed number>random value -> gn is greater than rv
// guessed number == random value -> Success! You won!

// use rand::Rng;
// use std::io;

// fn main() {
//     let random_value = rand::thread_rng().gen_range(1..=100);

//     println!("Guess the number between 1 and 100!");

//     loop {
//         let mut guessed_number = String::new();
//         io::stdin().read_line(&mut guessed_number).expect("Failed to read line");

//         let guessed_number: u32 = match guessed_number.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("Please input a valid number.");
//                 continue;
//             }
//         };

//         // if guessed_number < random_value {
//         //     println!("Your guess is less than random value!");
//         // } else if guessed_number > random_value {
//         //     println!("Your guess is greater than random value!");
//         // } else {
//         //     println!("Success! You won!");
//         //     break;
//         // }
//         match guessed_number.cmp(&random_value) {
//             std::cmp::Ordering::Less => println!("Your guess is less than random value!"),
//             std::cmp::Ordering::Greater => println!("Your guess is greater than random value!"),
//             std::cmp::Ordering::Equal => {
//                 println!("Success! You won!");
//                 break;
//             }
//         }
//     }
// }
//------------------------------------------------------------------------------------rendom-number-------------------------------------------------------------------------------------//

//------------------------------------------------------------------------------------countdown-------------------------------------------------------------------------------------//
//Countdown Rust
//input - 10
//10 9 8 7 6 5 ...1

// use std::io;
// use std::thread::sleep;
// use std::time::Duration;

// fn main() {
//     let mut input = String::new();
//     println!("Please enter the timer:");
//     io::stdin().read_line(&mut input).expect("Invalid Input");

//     let timer: u16 = input.trim().parse().expect("Invalid number");
//     start_timer(timer);
// }
// //1 2 3 4 5 6 .. 10 -> (1..=10)
// //10 9 8 7 6 5 ... 1 -> (1..=10).rev()
// fn start_timer(timer: u16) {
//     for i in (1..=timer).rev() {
//         println!("Timer countdown..{}", i);
//         sleep(Duration::from_secs(1)); //to have a delay of 1 second
//     }
// }
//------------------------------------------------------------------------------------countdown-------------------------------------------------------------------------------------//

//------------------------------------------------------------------------------------countdown-optimized------------------------------------------------------------------------------------//

//Countdown Rust
//input - 10
// 10 9 8 7 6 5 ...1

// use std::io;
// use std::thread::sleep;
// use std::time::Duration;

// fn main() {
//     loop {
//         let mut input = String::new();
//         println!("Please enter the timer:");
//         io::stdin().read_line(&mut input).expect("Invalid Input");

//         //let timer: u16 = input.trim().parse().expect("Invalid number");
//         let timer: u16 = match input.trim().parse() {
//             Ok(timer) => timer,
//             Err(_) => {
//                 println!("Invalid Number");
//                 continue;
//             }
//         };
//         start_timer(timer);
//         break;
//     }
// }
// //1 2 3 4 5 6 .. 10 -> (1..=10)
// //10 9 8 7 6 5 ... 1 -> (1..=10).rev()
// fn start_timer(timer: u16) {
//     for i in (1..=timer).rev() {
//         println!("Timer countdown..{}", i);
//         sleep(Duration::from_secs(1)); //to have a delay of 1 second
//     }
// }

// Flow of Control (Step-by-Step)
// The program enters the loop and asks the user to enter a timer value.
// The user enters a number:
// If it's valid → The number is stored, and start_timer(timer) starts.
// If it's invalid → "Invalid Number" is printed, and continue restarts the loop (asks for input again).
// After start_timer(timer) finishes, break stops the loop, and the program exits.
//------------------------------------------------------------------------------------countdown-optimized------------------------------------------------------------------------------------//

//------------------------------------------------------------------------------------binary-hex-converter------------------------------------------------------------------------------------//
// convert a natural number to binary number and hexadecimal number
//Converter

// use std::io;
// fn main() {
//     let mut input = String::new();
//     println!("Please enter a decimal value:");
//     io::stdin().read_line(&mut input).expect("Invalid input");

//     let decimal_val: u32 = match input.trim().parse() {
//         Ok(decimal_val) => decimal_val,
//         Err(_) => {
//             println!("Invalid number");
//             return;
//         }
//     };
//     binary_converter(decimal_val);
//     hex_converter(decimal_val);
// }

// fn binary_converter(mut decimal_val: u32) {
//     while decimal_val > 0 {
//         let remainder = decimal_val % 2;
//         print!("{}", remainder);
//         decimal_val = decimal_val / 2;
//     }
//     print!("\n");
// }
// fn hex_converter(mut decimal_val: u32) {
//     //10
//     while decimal_val > 0 {
//         let remainder: u8 = (decimal_val % 16) as u8;
//         if remainder > 9 {
//             print!("{}", (remainder + 55) as char);
//         } else {
//             print!("{}", remainder);
//         }
//         decimal_val = decimal_val / 16;
//     }
//     print!("\n");
// }
//------------------------------------------------------------------------------------binary-hex-converter------------------------------------------------------------------------------------//

//------------------------------------------------------------------------------------binary-hex-converter-optimized-------------------------------------------------------------------------------------//
//next project
// convert a natural number to binary number and hexadecimal number
//Converter

// use std::io;

// fn main() {
//     let mut input = String::new();
//     println!("Please enter a decimal value:");
//     io::stdin().read_line(&mut input).expect("Invalid input");

//     let decimal_val: u32 = match input.trim().parse() {
//         Ok(decimal_val) => decimal_val,
//         Err(_) => {
//             println!("Invalid number");
//             return;
//         }
//     };
//     binary_converter(decimal_val);
//     hex_converter(decimal_val);
// }

// fn binary_converter(mut decimal_val: u32) {
//     let mut binary = String::new();
//     while decimal_val > 0 {
//         let remainder: u8 = (decimal_val % 2) as u8;
//         binary.insert(0, (remainder + 48) as char); //0 1 0 1
//         //print!("{}", remainder);
//         decimal_val = decimal_val / 2;
//     }
//     println!("Binary Number:{}", binary);
//     print!("\n");
// }
// fn hex_converter(mut decimal_val: u32) {
//     let mut hex = String::new();
//     while decimal_val > 0 {
//         let remainder: u8 = (decimal_val % 16) as u8;
//         if remainder > 9 {
//             hex.insert(0, (remainder + 55) as char);
//             //print!("{}", (remainder + 55) as char);
//         } else {
//             hex.insert(0, (remainder + 48) as char);
//             //print!("{}", remainder);
//         }
//         decimal_val = decimal_val / 16;
//     }
//     println!("Hexadecimal Value:{}", hex);
// }
//------------------------------------------------------------------------------------binary-hex-converter-optimized-------------------------------------------------------------------------------------//

//-----------------------------------------------------------------------------------------banking-project-------------------------------------------------------------------------------------//
// use std::io;

// fn main() {
//     let mut user_balance: f64 = 0.0;
//     let mut flag = false;

//     loop {
//         println!("\nSimple Banking System");
//         println!("1. Deposit");
//         println!("2. Withdraw");
//         println!("3. Check Balance");
//         println!("4. Exit");

//         let mut input = String::new();
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Enter a valid input");

//         let choice: u8 = match input.trim().parse() {
//             Ok(choice) => choice,
//             Err(_) => {
//                 println!("Enter a valid choice");
//                 continue;
//             }
//         };

//         user_action(choice, &mut user_balance, &mut flag);

//         if flag {
//             break;
//         }
//     }
// }

// fn user_action(choice: u8, user_balance: &mut f64, flag: &mut bool) {
//     match choice {
//         1 => deposit(user_balance),
//         2 => withdraw(user_balance),
//         3 => check_balance(user_balance),
//         4 => {
//             *flag = exit();
//         }
//         //This true value is assigned to the flag variable through *flag = exit();.
//         _ => println!("Invalid choice"),
//     }
// }

// fn deposit(user_balance: &mut f64) {
//     println!("Please enter the amount to deposit");

//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Enter a valid input");

//     let amount: f64 = match input.trim().parse() {
//         Ok(amount) if amount > 0.0 => amount,
//         _ => {
//             println!("Please enter a positive number");
//             return;
//         }
//     };

//     *user_balance += amount;
//     println!("Deposited: ${:.2}", amount);
// }

// fn withdraw(user_balance: &mut f64) {
//     println!("Please enter the amount to withdraw");

//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Enter a valid input");

//     let amount: f64 = match input.trim().parse() {
//         Ok(amount) if amount > 0.0 => amount,
//         _ => {
//             println!("Please enter a positive number");
//             return;
//         }
//     };

//     if amount > *user_balance {
//         println!("Insufficient balance");
//         return;
//     }

//     *user_balance -= amount;
//     println!("Withdrew: ${:.2}", amount);
// }

// fn check_balance(user_balance: &f64) {
//     println!("Your current balance is: ${:.2}", user_balance);
// }

// fn exit() -> bool {
//     println!("Exiting the system. Goodbye!");
//     true
// }
//-----------------------------------------------------------------------------------------banking-project-------------------------------------------------------------------------------------//

//-----------------------------------------------------------------------------------------library-------------------------------------------------------------------------------------//
// #[derive(Debug)]
// struct Book {
//     title: String,
//     author: String,
//     is_available: bool,
// }

// #[derive(Debug)]
// struct Library {
//     book: Option<Book>,
// }

// impl Book {
//     fn borrow(&mut self) -> Result<(), String> {
//         if self.is_available {
//             self.is_available = false;
//             Ok(())
//         } else {
//             Err("Book is already borrowed".to_string())
//         }
//     }

//     fn return_book(&mut self) {
//         self.is_available = true;
//     }
// }

// impl Library {
//     fn add_book(&mut self, book: Book) {
//         self.book = Some(book);
//         println!("Book added to library!");
//     }

//     fn borrow_book(&mut self) -> Result<(), String> {
//         match self.book.as_mut() {
//             Some(book) => book.borrow(),
//             None => Err("No book available in library".to_string()),
//         }
//     }

//     fn return_book(&mut self) {
//         if let Some(book) = self.book.as_mut() {
//             book.return_book();
//             println!("Book returned!");
//         } else {
//             println!("No book to return.");
//         }
//     }
// }

// fn main() {
//     let book = Book {
//         title: "The Rust Book".to_string(),
//         author: "Steve Klabnik".to_string(),
//         is_available: true,
//     };

//     let mut library = Library { book: None };

//     library.add_book(book);

//     if let Err(err) = library.borrow_book() {
//         println!("Error: {}", err);
//     }

//     if let Err(err) = library.borrow_book() {
//         println!("Error: {}", err);
//     }

//     library.return_book();

//     if let Err(err) = library.borrow_book() {
//         println!("Error: {}", err);
//     }
// }

//-----------------------------------------------------------------------------------------library-------------------------------------------------------------------------------------//
