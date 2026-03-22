use std::collections::HashSet;

/*
  ⚙️  SET OPERATIONS FEEL:
  HashSet mein HUMEIN KEY KE SATH KOI VALUE NAHI CHAHIYE HOTA. ❌
  Bas hum check karte hain ki group mein kaun hai aur kaun nahi!

  HashSet aur HashMap mein yahi sabse bada fark hai:

HashSet (Value-less): Isme sirf Unique Keys hoti hain. Isme koi "Value" associated nahi hoti.
Example: Ek attendance list jaha sirf names hain.
HashMap (Key-Value Pair): Isme har unique key ke saath ek Value judi hoti hai.
Example: Ek phonebook jaha "Name" (Key) ke saath "Phone Number" (Value) juda hai.

Aapko jaan kar hairani hogi ki Rust internally HashSet ko ek HashMap ki tarah hi banata hai, bas wo Value wali jagah ko khali (()) chhod deta hai!

Aasan bhasha mein:

HashMap = { Key ➡️ Value }
HashSet = { Key }
*/

fn main() {
    // 🤝 Do alag-alag groups banate hain real-world scenario ke liye
    let mut concert_queue: HashSet<&str> = HashSet::new(); // Concert Group
    let mut movie_queue: HashSet<&str> = HashSet::new(); // Movie Group

    concert_queue.insert("Boris");
    concert_queue.insert("Melissa");

    movie_queue.insert("Boris");
    movie_queue.insert("Phil");

    // 🤝 UNION (Sabka Saath, Sabka Vikas):
    // Dono groups ke saare UNIQUE members. Jo dono mein hain, wo ek hi baar aayenge! 💎
    println!("Union: {:?}", concert_queue.union(&movie_queue));
    // Result: [Boris, Melissa, Phil]

    // 🔪 DIFFERENCE:
    // Concert group ke wo log jo Movie group mein NAHI hain. (Excluding common members like Boris)
    println!(
        "Difference (Concert - Movie): {:?}",
        concert_queue.difference(&movie_queue)
    );
    // Result: [Melissa] (Kyunki Boris Movie mein bhi hai!)

    println!(
        "Difference (Movie - Concert): {:?}",
        movie_queue.difference(&concert_queue)
    );
    // Result: [Phil]

    // 💎 SYMMETRIC DIFFERENCE (The "ONLY ONE" Feel):
    // Wo log jo ya toh sirf Concert mein hain, ya sirf Movie mein. (Common log gayab! 🚫)
    println!(
        "Symmetric Difference: {:?}",
        concert_queue.symmetric_difference(&movie_queue)
    );
    // Result: [Melissa, Phil]

    // 🧩 SUBSET & SUPERSET (The "Part of" Feel):
    let mut attendees = HashSet::new();
    attendees.insert("Boris");

    // 🔍 IS SUBSET: Kya "Boris" concert group ka ek hissa hai?
    println!(
        "Is Boris a subset of Concert? {}",
        attendees.is_subset(&concert_queue)
    ); // true

    // 🔍 IS SUPERSET: Kya Concert group "Boris" ko pura cover karta hai?
    println!(
        "Is Concert a superset of Boris? {}",
        concert_queue.is_superset(&attendees)
    ); // true
}
