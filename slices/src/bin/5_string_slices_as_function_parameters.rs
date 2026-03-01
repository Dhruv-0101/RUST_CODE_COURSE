fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day!");
}

fn main() {
    let action_hero = String::from("Arnold Schwarzenegger");
    do_hero_stuff(&action_hero);
    let another_action_hero = "Sylvester Stallone";
    do_hero_stuff(another_action_hero);
}
//derefcorrison
//agar &string ko as a fucntion parameter liye to woh sirf heap allocated string ko hi support karega
//agar &str ko as a fucntion parameter liye to woh heap allocated string aur &str ko both support karega