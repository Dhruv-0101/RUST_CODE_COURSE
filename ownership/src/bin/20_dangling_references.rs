fn main() {
    let city = create_city();
    println!("{city}");
}

fn create_city() -> String {
    String::from("New York")
}
// fn create_city() -> String {
//     let new_york = String::from("New York");
//     return new_york;
// }

// fn create_city() -> &String {
//     let city = String::from("New York");
//     &city // ERROR! `city` does not live long enough
// }


/*
✅ Safe Versions — No Dangling Reference
1. ✅ This version is safe:
rust
fn create_city() -> String {
    String::from("New York")
}
You're returning a String value, not a reference.

Ownership of the string is moved to the caller (main), so it's valid.

2. ✅ This version is also safe:
rust
fn create_city() -> String {
    let new_york = String::from("New York");
    return new_york;
}
Same logic: you're returning the owned String, not a reference.

Ownership is transferred, so no lifetime issues.

❌ Dangling Reference Version — Causes Error
3. ❌ This version causes a dangling reference:
rust
fn create_city() -> &String {
    let city = String::from("New York");
    &city // ERROR! `city` does not live long enough
}
You're returning a reference (&String) to a local variable.

city is dropped when the function ends, so the reference points to invalid memory.

Rust prevents this with a lifetime error.
 */


 /*
 🔍 What’s Happening in the Dangling Reference Case?
rust
fn create_city() -> &String {
    let city = String::from("New York");
    &city // ❌ Dangling reference!
}
Step-by-step breakdown:
let city = String::from("New York");

You created a String named city inside the function.

This lives only inside the function scope — i.e., between { ... }.

&city

You're trying to return a reference to city, not the actual String.

But city is a local variable — it will be dropped (freed from memory) as soon as the function ends.

Function ends → {} closes

Rust automatically drops city here.

Now you're returning a reference to something that no longer exists — this is called a dangling reference.

⚠️ Why Rust Throws Error
Rust says:

“Bhai, tu mujhe bol raha hai ki main ek reference return karu, lekin jis cheez ka reference hai wo toh khatam ho gaya! Main kaise valid rakhu usko?”

So Rust prevents this at compile time with a lifetime error.

✅ Safe Alternative
If you want to return the actual value, do this:

rust
fn create_city() -> String {
    String::from("New York") // Ownership is transferred, no lifetime issue
}
Here, String is moved to the caller (main), so no reference is needed — no risk of dangling.
  */