// =====================================================================
//  🗺️ EXPLORER AUR USKA KAAM — Closure Ka Asli Power
// =====================================================================
//
//  KAHANI:
//  -------
//  Ek NAKSHE (Map) ke paas 2 jagahein hain — "Enchanted Forest" aur "Mystic Mountain".
//  Naksha kehta hai:
//     "Main har jagah LE JAAUNGA tujhe — Lekin waha jaake KYA KARNA hai,
//      wo TU bata. Apna KAAM (Closure) de mujhe."
//
//  Pehli baar:  "Har jagah ka khazana gino"          (Closure #1)
//  Doosri baar: "Har jagah ka naam note karo"         (Closure #2)
//
//  SAME Naksha, SAME explore method, par KAAM alag alag! 🎯
//  Ye hai closure ka asli power — Logic Badlo, Method Mat Badlo.
//
// =====================================================================

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
    // ┌──────────────────────────────────────────────────────────────┐
    // │  NAKSHA KA EXPLORE METHOD 🧭                                │
    // │                                                              │
    // │  Naksha kehta hai:                                           │
    // │  "Mujhe ek KAAM (action) de. Main har jagah pe jaaunga      │
    // │   aur TERA diya hua kaam baar baar chalaunga."               │
    // │                                                              │
    // │  action: F  where F: FnMut(&Location)                       │
    // │  ^^^^^^           ^^^^^                                     │
    // │  tera kaam        BAAR BAAR chale + bahar ki cheezein       │
    // │                   BADAL sake (isliye FnMut, na ki FnOnce)   │
    // │                                                              │
    // │  &self: Naksha khatam NAHI hoga, explore ke baad bhi rahega  │
    // │         (isliye & lagaya, bina & hota to ek baar mein khatam)│
    // └──────────────────────────────────────────────────────────────┘
    fn explore<F>(&self, mut action: F)
    where
        F: FnMut(&Location),
    {
        println!("🗺️ Starting Map Exploration...");
        for current_location in self.locations {
            action(current_location); // <-- HAR jagah pe TERA kaam chala diya
        }
        println!("🏁 Exploration Finished.");
    }
}

fn main() {
    // ── STEP 1: Jagahein banayi ──
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
    // ── STEP 2: Naksha banaya jo in jagahon ko jaanta hai ──
    let map = Map {
        locations: &locations,
    };

    // =================================================================
    //  TASK 1: KHAZANA GINO 💰
    // =================================================================
    //
    //  Dekh bhai:
    //  total_treasures BAHAR hai (main mein)
    //  par closure ANDAR se isko badal raha hai.
    //
    //  Ye tabhi possible hai jab closure FnMut ho,
    //  kyunki FnMut kehta hai: "Main bahar ki cheezein MODIFY kar sakta hoon"
    //
    let mut total_treasures = 0;

    map.explore(|loc| {
        //  Ye closure HAR JAGAH ke liye chalega —
        //  pehle Forest ke liye, phir Mountain ke liye.
        println!("Robot: Found {} treasures at {}!", loc.treasures, loc.name);
        total_treasures += loc.treasures; // <-- BAHAR ka variable ANDAR se badla!
    });

    println!("Total pieces collected: {}", total_treasures);

    println!("\n--- New Task ---");

    // =================================================================
    //  TASK 2: NAAM COLLECT KARO 📝
    // =================================================================
    //
    //  SAME map.explore() method hai — ek bhi line nahi badli uski.
    //  Par ab KAAM bilkul alag hai!
    //
    //  Pehle khazana gin rahe the, ab naam collect kar rahe hain.
    //  Ye FLEXIBILITY sirf closure ki wajah se hai.
    //
    let mut names = Vec::new();

    map.explore(|loc| {
        //  Is baar Robot treasure nahi gin raha,
        //  bas har jagah ka naam apni list mein daal raha hai.
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
=====================================================================
  🎬 POORA EXECUTION FLOW — Step by Step

  main() shuru hua
       │
       ▼
  2 Locations bani (Forest, Mountain)
       │
       ▼
  Map banaya (naksha in locations ko jaanta hai)
       │
       │
  ═══ TASK 1: Khazana Gino ═══
       │
       ▼
  total_treasures = 0  (BAHAR define kiya)
       │
       ▼
  map.explore( |loc| { total_treasures += loc.treasures } )
       │
       ├──► Naksha LOOP shuru karta hai
       │         │
       │         ├── loc = Forest  ──► Closure chala ──► total = 0 + 5  = 5
       │         │
       │         └── loc = Mountain ──► Closure chala ──► total = 5 + 10 = 15
       │
       ▼
  Print: "Total pieces collected: 15"
       │
       │
  ═══ TASK 2: Naam Collect ═══
       │
       ▼
  names = []  (khaali list)
       │
       ▼
  map.explore( |loc| { names.push(loc.name) } )
       │
       ├──► SAME Naksha, SAME loop, par ALAG closure
       │         │
       │         ├── loc = Forest  ──► Closure chala ──► names = ["Enchanted Forest"]
       │         │
       │         └── loc = Mountain ──► Closure chala ──► names = ["Enchanted Forest", "Mystic Mountain"]
       │
       ▼
  Print: Places visited: ["Enchanted Forest", "Mystic Mountain"]

=====================================================================

  🤔 CLOSURE KA FAYDA YE HAI:

  explore() method EK hi baar likha.
  Par 2 ALAG ALAG kaam karwa liye:
    1. Khazana gino
    2. Naam collect karo

  Agar closure nahi hota to?
    → explore_and_count_treasures() alag method banana padta
    → explore_and_collect_names()   alag method banana padta
    → Har naye kaam ke liye NAI method — GANDA kaam!

  Closure se:
    Naksha bolta hai "TU bata kya karna hai, main loop chalata hoon."
    Bas! Kaam badlo, method mat badlo. CLEAN kaam! 🎯

=====================================================================

  🔑 FnOnce vs FnMut ka FARAK (Pichle file se compare karo):

  Pichli file (Tijori):
    → FnOnce: Closure EK BAAR chala aur khatam (Tijori bhi khatam)
    → Password ek baar chahiye tha, bas.

  Ye file (Naksha):
    → FnMut: Closure BAAR BAAR chala (har location pe)
    → Aur BAHAR ke variables (total_treasures, names) ko MODIFY bhi kiya
    → Isliye FnMut chahiye, FnOnce se kaam nahi banta yaha!

=====================================================================
*/
