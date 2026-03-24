// =====================================================================
//  🏦 TIJORI AUR CHOR — Closure Ka Asli Feel
// =====================================================================
//
//  KAHANI:
//  -------
//  Ek Tijori (Vault) hai jisme "Gold" rakha hai.
//  Tijori ke andar ek Robot Guard baitha hai.
//  Robot kehta hai:
//     "Mujhe password lana nahi aata. Tu mujhe ek TARIKA (procedure) de,
//      main us tarike se password nikaalu, phir match karunga."
//
//  Wo "TARIKA" hi Closure hai. 🎯
//
//  Matlab:  Tijori ko SIRF ye pata hai ki password CHECK kaise karna hai.
//           Password LANE ka kaam — wo TERE upar hai bhai!
//
// =====================================================================

use std::io::stdin;

#[derive(Debug)]
struct Vault {
    password: String,
    treasure: String,
}

impl Vault {
    // ┌─────────────────────────────────────────────────────────┐
    // │  ROBOT GUARD KA METHOD                                  │
    // │                                                         │
    // │  Robot kehta hai:                                        │
    // │  "Mujhe ek TARIKA (procedure) de jo password laaye.     │
    // │   Main usey ek hi baar chalaunga, check karunga, khatam."│
    // │                                                         │
    // │  procedure: impl FnOnce() -> String                     │
    // │  ^^^^^^^^   ^^^^^^^^^^^^^^^^^^^^                        │
    // │  koi bhi    jo EK BAAR chale aur String de              │
    // │  tarika                                                 │
    // │                                                         │
    // │  self (bina &): Tijori khulte hi KHATAM. Dobara nahi    │
    // │                 khul sakti. One-time use.                │
    // └─────────────────────────────────────────────────────────┘
    fn unlock(self, procedure: impl FnOnce() -> String) -> Option<String> {
        // STEP A: Robot ab TERA diya hua tarika (closure) chala raha hai
        let password = procedure(); // <-- YAHI hai closure ka kaam!
        //     Robot ne "procedure()" bola
        //     aur Closure ne password laake de diya.

        // STEP B: Ab Robot match karega
        if password == self.password {
            println!("Robot: Password Match! Access Granted.");
            Some(self.treasure) // ✅ Sahi hai — Gold le jaa bhai
        } else {
            println!("Robot: WRONG! Alarm triggered 🚨");
            None // ❌ Galat — kuch nahi milega
        }
    }
}

fn main() {
    // ── STEP 1: Tijori banayi ──
    let vault = Vault {
        password: String::from("topsecret"),
        treasure: String::from("Gold"),
    };

    // ── STEP 2: Apna TARIKA (Closure) ready kiya ──
    //
    //  Ye closure hai — ek "packed logic" jo abhi CHALA nahi,
    //  bas READY hai. Jab koi isko call karega TAB chalega.
    //
    //  Soch aise: tune ek CHIT likh ke rakhi hai jisme likha hai
    //  "user se input maango aur return karo". Chit abhi band hai.
    //
    let hack = || {
        let mut user_input = String::new();
        println!("Robot: Enter password to crack the vault:");
        stdin().read_line(&mut user_input).expect("Failed to read");
        user_input.trim().to_string()
    };
    // hack abhi tak CHALA nahi hai! Sirf define hua hai.

    // ── STEP 3: Tijori ko apna Tarika (Closure) de diya ──
    //
    //  vault.unlock(hack)
    //  ^^^^^ ^^^^^^ ^^^^
    //  │     │      └── ye hamara closure hai (password lane ka tarika)
    //  │     └── Robot Guard ka method
    //  └── Tijori
    //
    //  Ab kya hoga:
    //    → Robot ko "hack" mil gaya
    //    → Robot ne hack() call kiya  (STEP A upar)
    //    → User se input aaya
    //    → Robot ne match kiya        (STEP B upar)
    //    → Result aaya: Some("Gold") ya None
    //
    let extraction = vault.unlock(hack);

    println!("Bhai, result ye raha: {:?}", extraction);
}

/*
EXPECTED OUTPUT (If you type 'topsecret'):
Robot: Enter password to crack the vault:
topsecret
Robot: Password Match! Access Granted.
Bhai, result ye raha: Some("Gold")

EXPECTED OUTPUT (If you type wrong):
Robot: Enter password to crack the vault:
admin123
Robot: WRONG! Alarm triggered 🚨
Bhai, result ye raha: None
*/

/*
=====================================================================
  🎬 POORA EXECUTION FLOW — Ek Baar Dimaag Mein Chala

  main() shuru hua
       │
       ▼
  Tijori bani (password="topsecret", treasure="Gold")
       │
       ▼
  hack closure DEFINE hua (abhi chala nahi, sirf chit likhi)
       │
       ▼
  vault.unlock(hack) call hua
       │
       ├──► Robot ke paas "hack" closure aaya
       │
       ├──► Robot ne procedure() bola ──► CLOSURE CHALA! ──► User se input liya
       │                                                          │
       │                            password = "jo bhi user ne likha" ◄──┘
       │
       ├──► Robot ne check kiya: password == "topsecret" ?
       │         │                    │
       │        YES                  NO
       │         │                    │
       │    Some("Gold")            None
       │         │                    │
       ▼         ▼                    ▼
  extraction mein result aa gaya. Print. Khatam.

=====================================================================

  🤔 AGAR CLOSURE NAHI HOTA TO?

  Tab Robot ke andar hi likhna padta:
      "user se stdin se input lo, trim karo, return karo"

  Lekin kal ko agar password FILE se lana ho? Ya DATABASE se?
  Har baar Robot ka code change karna padega. GANDA kaam.

  Closure se Robot FLEXIBLE ho gaya:
      "Mujhe mat batao kaise lana hai. Tu apna tarika de,
       main bas ek baar chala ke result le lunga."

  YE hai closure ka ASLI FAYDA — Logic Ko Alag Rakh Sakte Ho! 🎯
=====================================================================
*/
