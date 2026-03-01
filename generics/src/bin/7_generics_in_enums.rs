enum Cheesesteak<T> {
    Plain,
    Topping(T),
}

fn main() {
    let mushroom = Cheesesteak::Topping("mushroom");
    let onions = Cheesesteak::Topping("onions".to_string()); //heap allocated(String)
    let topping = "bacon".to_string();
    let bacon = Cheesesteak::Topping(&topping);

    let mut plain: Cheesesteak<String> = Cheesesteak::Plain;

    // Invalid, &str is not a String, which is what T must be for plain variable//above line both have same type for generics
    plain = Cheesesteak::Topping("sausage");
}
//yaha pe dekho hamne pehle mushroom onions toppings bacon ko jo type de ne the de diye right.ab dekho plain variable ko hamne explicitly type de diya ki wo Cheesesteak<String> hoga, iska matlab hai ki plain variable ke liye T ka type String hoga. Ab jab ham plain variable ko Cheesesteak::Topping("sausage") se assign karne ki koshish karte hain, to compiler error dega kyunki "sausage" ek &str hai, na ki String. Isliye, plain variable ke liye T ka type String hona chahiye, aur "sausage" ko String me convert karna hoga agar ham usse plain variable me assign karna chahte hain.