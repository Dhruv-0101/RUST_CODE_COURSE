#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String, String),
}

fn main() {
    let mut my_payment_method = PaymentMethodType::CreditCard(String::from("0034-5678-9012-3456"));

    my_payment_method =
        PaymentMethodType::PayPal(String::from("bob@email.com"), String::from("password"));

    println!("{:?}", my_payment_method);
}
/*
🔢 Step-by-Step Byte Analysis
📦 String in Rust (on 64-bit system):
A String is a struct with 3 fields:

A pointer to the heap data (*const u8) → 8 bytes

Length (usize) → 8 bytes

Capacity (usize) → 8 bytes

👉 So, 1 String = 24 bytes on a 64-bit system.
*/

/*
🔘 Variant Sizes:
Variant	Data	            Size
CreditCard	   1 String	  24 bytes
DebitCard	   1 String	  24 bytes
PayPal	       2 Strings  48 bytes
*/

/*
📦 Final Enum Size:
Rust allocates memory for:

The largest variant → PayPal(String, String) = 48 bytes

the discriminant (tag) = 8 bytes

🔸 Total size of PaymentMethodType = 48 + 8 = 56 bytes
*/

/*
Rust enums store only ONE variant at a time, not all of them together.
🧩 Think of it like this:
An enum is like a box that holds only one of its variants at any moment.

So it doesn’t need space for all variants — only the largest one, plus the discriminant (tag).
*/

/*
let visa = PaymentMethodType::CreditCard(String::from("0034-5678-9012-3456"));
let mastercard = PaymentMethodType::DebitCard(String::from("2532-1298-2093-4800"));

Each variable (visa and mastercard) is its own value of type PaymentMethodType.
So yes — both exist at the same time, but each one is a separate enum instance.
*/

/*
❗ But:
If you did this instead:
let mut payment = PaymentMethodType::CreditCard(...);
payment = PaymentMethodType::DebitCard(...); // 🔁 replaces old value
Then payment holds only one value at a time, and the first is dropped when replaced.
*/

/*
📝 So:
let my_payment_method = PaymentMethodType::CreditCard(...);
➡️ Allocates 32 bytes for that variable.

🔁 If you do this too:
let my_payment_method_2 = PaymentMethodType::DebitCard(...);
➡️ Another 32 bytes is allocated in memory for this second variable.
*/
