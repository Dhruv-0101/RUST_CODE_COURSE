#[derive(Debug)]
struct DeliSandwich {}

fn main() {
    println!("{}", identity::<i32>(5));
    println!("{}", identity::<i8>(5));
    println!("{}", identity::<u32>(5));
    println!("{}", identity::<f64>(13.14));
    println!("{}", identity::<&str>("hello"));
    println!("{}", identity::<String>(String::from("hello")));
    println!("{}", identity::<bool>(true));
    println!("{:?}", identity::<DeliSandwich>(DeliSandwich {}));
}

fn identity<T>(value: T) -> T {
    value
}
//nothing just a demonstration of the turbofish operator, which is used to specify the type parameter when calling a generic function. In this case, we are calling the `identity` function with different types of arguments, and we use the turbofish operator to specify the type parameter for each call.