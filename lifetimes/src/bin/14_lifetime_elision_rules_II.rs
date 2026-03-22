// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 13)
// ==============================

// Topic: Lifetime Elision Rule 3 (The &self Rule)
// Feel: 
// "Bhai, Rule 3 sabse common hai methods ke liye. 
// Ye kehta hai: 'Agar kisi method mein &self parameter hai, 
// toh return reference bhi HAMESHA &self ki lifetime lega!'"

struct Book {
    title: String,
}

impl Book {
    // Feel: Yaha logic ye hai ki title khud 'self' ka ek part hai.
    // Isliye 'self' ka zinda rehne par title valid rahega.
    // Rust isse automatically handles kar leta hai!
    fn get_title(&self, prefix: &str) -> &str {
        println!("Displaying with prefix: {}", prefix);
        &self.title
    }
}

fn main() {
    let my_book = Book {
        title: String::from("Rust Adventures"),
    };

    // 'self' (my_book) zinda hai, toh 'title_ref' bhi valid hai.
    let title_ref = my_book.get_title("📖 Book:");
    println!("Title: {}", title_ref);

    /* 🛑 ERROR: 
    let another_ref;
    {
        let temp_book = Book { title: String::from("Ghost") };
        another_ref = temp_book.get_title("Wait:");
    } // temp_book is gone!
    // println!("{}", another_ref); // ❌ Dangling pointer!
    */

}

/*
EXPECTED OUTPUT:
Displaying with prefix: 📖 Book:
Title: Rust Adventures
*/

/*
🧠 Deep Feel:
Rule 3 methods ko cleaner banta hai. 
Jab aap `&self` lene lagte ho, compiler assume kar leta hai ki
'Bhai, return wali value self ke andar se hi kuch hogi.'
Isliye wo `'a` khud use karta hai self and return ke liye. 
Precedence: Rule 3 Rule 1 aur Rule 2 se zyada important hai. 
Agar multiple parameters ho BUT &self bhi ho, toh &self wins!
*/
