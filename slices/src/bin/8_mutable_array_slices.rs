// fn main() {
//     let mut my_array = [10, 15, 20, 25, 30];
//     let my_slice: &mut [i32] = &mut my_array[2..4];
//     println!("My slice: {:?}", my_slice);

//     my_slice[0] = 100;
//     println!("My slice: {:?}", my_slice);
//     println!("My array: {:?}", my_array);
// }
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    // s.clear(); // error!

    println!("the first word is: {word}");
}
/*
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5------------//yaha pe find first word ne ref to le liya aur hamne ref ki computed value return kardi.aur usse hua kya ref out of scope chala gaya aur me usse wapas se use kar sakt ahoon

    //uska ref valid rehna chahiye bhai

    s.clear(); // this empties the String, making it equal to ""

    println!("For the word s: {}, the first word is: {}", s, word);

    // `word` still has the value `5` here, but `s` no longer has any content
    // that we could meaningfully use with the value `5`, so `word` is now
    // totally invalid!
}
*/
