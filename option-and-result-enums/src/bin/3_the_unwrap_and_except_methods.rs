fn main() {
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass: Option<&String> = musical_instruments.get(2);
    println!("{:?}", bass);
    println!("hello");
    let valid_instrument = bass.expect("Unable to retrieve element");
    // let valid_instrument = bass.unwrap();//mil gaya to to woh option ka value dega means here bass likr that[some(Bass)]
    println!("{valid_instrument}");

    let invalid_instrument = musical_instruments.get(100);
    println!("{:?}", invalid_instrument);
    // let valid_instrument = bass.unwrap();//error
    invalid_instrument.expect("Unable to retrieve musical instrument");
}
