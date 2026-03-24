// =====================================================================
//  🚪 VIP CLUB AUR BOUNCER — .retain() Mein Closure Ka Kamaal
// =====================================================================
//
//  KAHANI:
//  -------
//  Ek VIP Club hai jiska naam hai "PlayStation".
//  Club mein 11 log khade hain: P, l, a, y, S, t, a, t, i, o, n
//
//  Ab Club ne ek BOUNCER (Closure) rakha hai gate pe.
//  Bouncer ka ek hi kaam hai:
//     "Har aadmi ko dekho. Agar wo 'a' hai to BAHAR nikalo.
//      Baaki sabko ANDAR rehne do."
//
//  .retain() kehta hai:
//     "Main ek ek character ko tere paas launga, tu bol — RAKHUN ya NIKAALUN?"
//     true  = "Andar rakh"
//     false = "Bahar nikaal"
//
//  YE hai closure ka kaam — DECISION DENA. 🎯
//
// =====================================================================

fn main() {
    // ── STEP 1: Club bana jisme sab log (characters) khade hain ──
    let mut game_console = String::from("PlayStation");

    // ── STEP 2: Exit door pe ek khaali register rakha ──
    //  Jisko bhi bahar nikalenge, uska naam yaha likhenge.
    let mut kicked_out = String::new();

    // ── STEP 3: BOUNCER (Closure) tayaar kiya ──
    //
    //  Ye closure hai — ek "Decision Machine" jo har character ke liye
    //  bataegi: ANDAR rakh (true) ya BAHAR nikaal (false)?
    //
    //  Dhyaan se dekh:
    //  kicked_out BAHAR define hua hai (Step 2 mein)
    //  par Bouncer ANDAR se isko badal raha hai (push kar raha hai)
    //  → Isliye ye FnMut hai (bahar ki cheez modify kar sakta hai)
    //
    let bouncer = |guest: char| {
        if guest != 'a' {
            true // ✅ "Tu theek hai, andar reh."
        } else {
            kicked_out.push(guest); // 📝 Pehle register mein naam likha
            false // ❌ "Nikal bahar!"
        }
    };

    // ── STEP 4: .retain() ne Bouncer ko kaam pe lagaya ──
    //
    //  .retain() KHUD ek loop chalata hai — har character ko uthata hai
    //  aur TERE Bouncer (closure) ke paas le jaata hai.
    //
    //  Tu sirf DECISION de raha hai (true/false),
    //  baaki string se hatane ka kaam .retain() khud karta hai.
    //
    game_console.retain(bouncer);

    println!("--- Club Status ---");
    println!("VIPs remained in club (game_console): {}", game_console);
    println!("People at the exit door (kicked_out): {}", kicked_out);
}

/*
EXPECTED OUTPUT:
--- Club Status ---
VIPs remained in club (game_console): PlySttion
People at the exit door (kicked_out): aa
*/

/*
=====================================================================
  🎬 POORA EXECUTION FLOW — Ek Ek Character Ka Safar

  game_console = "PlayStation"
  kicked_out   = ""

  .retain(bouncer) shuru hua — har character ko Bouncer ke paas le gaya:

  Character │ Bouncer ka Decision │ Club mein raha? │ kicked_out
  ──────────┼─────────────────────┼─────────────────┼───────────
     'P'    │ 'a' nahi hai → true │ ✅ Haan          │ ""
     'l'    │ 'a' nahi hai → true │ ✅ Haan          │ ""
     'a'    │ 'a' HAI → false     │ ❌ BAHAR!        │ "a"
     'y'    │ 'a' nahi hai → true │ ✅ Haan          │ "a"
     'S'    │ 'a' nahi hai → true │ ✅ Haan          │ "a"
     't'    │ 'a' nahi hai → true │ ✅ Haan          │ "a"
     'a'    │ 'a' HAI → false     │ ❌ BAHAR!        │ "aa"
     't'    │ 'a' nahi hai → true │ ✅ Haan          │ "aa"
     'i'    │ 'a' nahi hai → true │ ✅ Haan          │ "aa"
     'o'    │ 'a' nahi hai → true │ ✅ Haan          │ "aa"
     'n'    │ 'a' nahi hai → true │ ✅ Haan          │ "aa"

  RESULT:
  game_console = "PlySttion"   (Club mein jo bache)
  kicked_out   = "aa"          (Jo bahar nikale gaye)

=====================================================================

  🤔 CLOSURE KA FAYDA YE HAI:

  .retain() Rust ki standard library ka method hai.
  Isko SIRF ek cheez chahiye — ek DECISION MACHINE (closure).
  Tu bol: "ye character rakhun ya hataaun?"

  Agar closure nahi hota to?
    → .retain() exist hi nahi kar paata is form mein!
    → Tujhe manually ek naya String banana padta,
      loop chalana padta, ek ek character check karna padta,
      aur phir sab copy karna padta. GANDA kaam.

  Closure se:
    .retain() bolta hai "Logic TU de, hatane ka kaam MAIN karunga."
    Ek line mein kaam ho gaya! 🎯

=====================================================================

  🔑 PICHLI FILES SE COMPARE:

  File 1 (Tijori):
    → FnOnce: Closure EK BAAR chala. Password liya, khatam.

  File 2 (Naksha):
    → FnMut: Closure BAAR BAAR chala. Bahar ki cheezein modify ki.

  YE File (VIP Club):
    → FnMut: Closure BAAR BAAR chala (har character ke liye)
    → Bahar ka `kicked_out` modify kiya (isliye FnMut chahiye)
    → .retain() Rust ki built-in method hai jo CLOSURE maangti hai
      Matlab Rust KHUD closure use karta hai apne standard library mein!

=====================================================================
*/
