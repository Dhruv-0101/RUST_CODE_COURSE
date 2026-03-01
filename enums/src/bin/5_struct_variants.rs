#[derive(Debug)]
// struct Credentials {
//     username: String,
//     password: String,
// }
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    // Paypal(Credentials),
    PayPal { username: String, password: String },
    Cash,
}

#[allow(unused_variables)]
fn main() {
    let visa = PaymentMethodType::CreditCard(String::from("0012-3456"));

    let paypal = PaymentMethodType::PayPal {
        username: String::from("bob@gmail.com"),
        password: String::from("password"),
    };
    println!("{:?}", paypal);
}
