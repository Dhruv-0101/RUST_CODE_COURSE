// =====================================================================
//  🏢 MANAGER AUR TEEN BAAR KA KAAM — FnMut Trait Ka Feel
// =====================================================================
//
//  KAHANI:
//  -------
//  Ek MANAGER (function) hai `execute_thrice`.
//  Manager kehta hai:
//     "Mujhe ek KAAM (closure) de. Main usey TEEN BAAR chalaunga."
//
//  Ab tere paas ek closure hai jo `bosses` list mein
//  naya boss "Alexandra" add karta hai.
//
//  Teen baar chalega → teen baar add hoga → list badh jayegi!
//
//  Par yaha DIKKAT ye hai:
//     Closure BAHAR ki list (bosses) ko BADAL raha hai.
//     Isliye iska CONTRACT hoga → FnMut
//     (Fn nahi chalega kyunki Fn sirf DEKHTA hai, BADALTA nahi)
//
// =====================================================================

// ┌──────────────────────────────────────────────────────────────┐
// │  MANAGER KA FUNCTION                                        │
// │                                                              │
// │  Manager kehta hai:                                          │
// │  "Mujhe koi bhi KAAM de jo FnMut() ho,                      │
// │   main usey 3 baar chalaunga."                               │
// │                                                              │
// │  F: FnMut()                                                  │
// │     ^^^^                                                     │
// │     BAAR BAAR chale + bahar ki cheezein BADAL sake            │
// │                                                              │
// │  Kyun FnMut aur FnOnce nahi?                                 │
// │  → FnOnce sirf EK BAAR chalta hai                            │
// │  → Yaha 3 baar chahiye, to FnMut zaroori hai!                │
// └──────────────────────────────────────────────────────────────┘
fn execute_thrice<F>(mut procedure: F)
where
    F: FnMut(),
{
    println!("--- Manager is triggering the procedure ---");
    procedure(); // 1st baar
    procedure(); // 2nd baar
    procedure(); // 3rd baar
}

fn main() {
    // ── STEP 1: Boss list banayi, abhi sirf ek boss hai ──
    let mut bosses = vec!["Boris"];

    // ── STEP 2: Closure banaya — "naya boss add karo" ──
    //
    //  Ye closure BAHAR ki `bosses` list ko MODIFY kar raha hai.
    //  → Isliye ye automatically FnMut ban gaya.
    //
    //  Agar ye sirf bosses PADH raha hota (println) → Fn hota
    //  Agar ye bosses KHAATA (move) → FnOnce hota
    //  Par ye bosses BADAL raha hai (push) → FnMut hai ✅
    //
    let closure = || {
        println!("Robot adding a boss...");
        bosses.push("Alexandra"); // <-- BAHAR ki list ANDAR se badli!
    };

    // ── STEP 3: Manager ko closure de diya ──
    //
    //  Manager ne 3 baar call kiya:
    //    1st → bosses = ["Boris", "Alexandra"]
    //    2nd → bosses = ["Boris", "Alexandra", "Alexandra"]
    //    3rd → bosses = ["Boris", "Alexandra", "Alexandra", "Alexandra"]
    //
    execute_thrice(closure);

    println!("Total Bosses: {:?}", bosses);
}

/*
EXPECTED OUTPUT:
--- Manager is triggering the procedure ---
Robot adding a boss...
Robot adding a boss...
Robot adding a boss...
Total Bosses: ["Boris", "Alexandra", "Alexandra", "Alexandra"]
*/

/*
=====================================================================
  🎬 POORA EXECUTION FLOW

  main() shuru hua
       │
       ▼
  bosses = ["Boris"]  (ek boss hai)
       │
       ▼
  closure DEFINE hua (abhi chala nahi, sirf tayaar hai)
       │
       ▼
  execute_thrice(closure) call hua
       │
       ├──► Manager ke paas closure aaya
       │
       ├──► procedure()  ──► 1st baar ──► bosses = ["Boris", "Alexandra"]
       │
       ├──► procedure()  ──► 2nd baar ──► bosses = ["Boris", "Alexandra", "Alexandra"]
       │
       ├──► procedure()  ──► 3rd baar ──► bosses = ["Boris", "Alexandra", "Alexandra", "Alexandra"]
       │
       ▼
  Print: Total Bosses: ["Boris", "Alexandra", "Alexandra", "Alexandra"]

=====================================================================

  🔑 Fn vs FnMut vs FnOnce — Ek Hi Jagah Samjho:

  ┌─────────┬──────────────────────────────┬──────────────┐
  │  Trait   │  Kya karta hai               │  Kitni baar  │
  ├─────────┼──────────────────────────────┼──────────────┤
  │  Fn     │  Sirf DEKHTA hai (read-only)  │  Unlimited   │
  │  FnMut  │  BADALTA hai (modify)         │  Unlimited   │
  │  FnOnce │  KHAATA hai (consume/move)    │  Sirf 1 baar │
  └─────────┴──────────────────────────────┴──────────────┘

  Is code mein:
    Closure `bosses.push()` kar raha hai → BADAL raha hai → FnMut ✅
    Manager 3 baar call karta hai → FnOnce se kaam nahi banega ❌
    Isliye FnMut PERFECT hai yaha! 🎯

=====================================================================
*/
