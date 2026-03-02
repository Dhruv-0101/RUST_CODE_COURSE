// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 9)
// ==============================

// Topic: Where Clauses
// Feel:
// "Bhai, jab traits ki line bohot lambi ho jati hai (T: Trait1 + Trait2 + Trait3),
// toh code ganda dkhne lagta hai.
// 'where' clause aata hai aur bolta hai: 'Shanti rakho, saari conditions niche ek saath saaf-saaf likh dete hain!'"

trait Accommodation {
    fn book(&mut self);
}
trait Insurance {
    fn get_policy(&self);
}
trait Review {
    fn get_rating(&self);
}

struct GrandHotel;
impl Accommodation for GrandHotel {
    fn book(&mut self) {
        println!("Booked!");
    }
}
impl Insurance for GrandHotel {
    fn get_policy(&self) {
        println!("Insured!");
    }
}
impl Review for GrandHotel {
    fn get_rating(&self) {
        println!("5 Stars!");
    }
}

// ==============================
// 1. GHATIYA TARIKA (Hard to read)
// ==============================
fn complex_function_noisy<T: Accommodation + Insurance + Review>(place: &mut T) {
    place.book();
}

// ==============================
// 2. BADHIYA TARIKA (Using 'where')
// ==============================
// Feel: "Function signature clean ho gaya, conditions niche 'where' mein hain."
fn complex_function_clean<T>(mut place: T)
where
    T: Accommodation + Insurance + Review,
{
    println!("--- Processing Complex Booking ---");
    place.get_policy();
    place.get_rating();
    place.book();
}

fn main() {
    let hotel = GrandHotel;

    // Call the clean version
    complex_function_clean(hotel);
}

/*
EXPECTED OUTPUT:
--- Processing Complex Booking ---
Insured!
5 Stars!
Booked!
*/

/*
🧠 Deep Feel:
'where' clause sirf readability ke liye nahi hai,
kabhi kabhi specific generic logic (like T: Iterator, T::Item: Display)
ke liye ye mandatory ho jata hai.
Short bounds ke liye `: Trait` use karo, complex ke liye `where`!
*/
