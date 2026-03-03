// ==============================
// RUST CLOSURES — FULL FEEL VERSION (EPISODE 12)
// ==============================

// Topic: Custom Iteration with Closures (The Explorer)
// Feel:
// "Bhai, ye closure ka 'Asli Power' dikhata hai.
// Socho aapke paas ek Map hai. Map ke paas logic hai 'Jagah Jagah Jaane' ka.
// Par har jagah jaakar kya 'KAAM' karna hai, wo aap closure ke zariye naya naya bhej sakte ho!"

#[derive(Debug)]
struct Location {
    name: String,
    treasures: u32,
}

#[derive(Debug)]
struct Map<'a> {
    locations: &'a [Location],
}

impl<'a> Map<'a> {
    // ------------------------------
    // THE EXPLORE METHOD 🧭
    // ------------------------------
    // Feel: Ye method sirf 'Loop' chalana janta hai.
    // Ye har location ko `action` (closure) ke hawale kar deta hai.
    fn explore<F>(&self, mut action: F)
    where
        F: FnMut(&Location), // Shart: "Bhai, location dekho aur kuch bhi badlo!"
    {
        println!("🗺️ Starting Map Exploration...");
        for current_location in self.locations {
            // Robot (action) ko current location de di
            action(current_location);
        }
        println!("🏁 Exploration Finished.");
    }
}

fn main() {
    let locations = [
        Location {
            name: String::from("Enchanted Forest"),
            treasures: 5,
        },
        Location {
            name: String::from("Mystic Mountain"),
            treasures: 10,
        },
    ];
    let map = Map {
        locations: &locations,
    };

    // ------------------------------
    // TASK 1: COUNT TREASURES 💰
    // ------------------------------
    let mut total_treasures = 0;
    // Feel: Map sirf jagah jagah ja raha hai, counting hum closure mein kar rahe hain.
    map.explore(|loc| {
        println!("Robot: Found {} treasures at {}!", loc.treasures, loc.name);
        total_treasures += loc.treasures;
    });
    println!("Total pieces collected: {}", total_treasures);

    println!("\n--- New Task ---");

    // ------------------------------
    // TASK 2: COLLECT NAMES 📝
    // ------------------------------
    let mut names = Vec::new();
    // Same 'map.explore' method, par ab kaam badal gaya!
    map.explore(|loc| {
        names.push(loc.name.clone());
    });
    println!("Places visited: {:?}", names);
}

/*
EXPECTED OUTPUT:
🗺️ Starting Map Exploration...
Robot: Found 5 treasures at Enchanted Forest!
Robot: Found 10 treasures at Mystic Mountain!
🏁 Exploration Finished.
Total pieces collected: 15

--- New Task ---
🗺️ Starting Map Exploration...
🏁 Exploration Finished.
Places visited: ["Enchanted Forest", "Mystic Mountain"]
*/

/*
🧠 Deep Feel:
1. Reusability: `explore` method ko baar baar badalne ki zaroorat nahi.
   Uski 'Looping' logic fixed hai, lekin uska 'Behavior' generic hai.
2. FnMut Power: Dono hi tasks mein humne bahar ke variables (`total_treasures`, `names`)
   ko change kiya. Isiliye `FnMut` shart (bound) kaam aayi.
3. Separation: Data (Map) aur Action (Closure) alag alag hain.

Bhai, ye ek 'Auto-Pilot' car hai jisne aap driver (closure) ko bitha diya hai.
*/
