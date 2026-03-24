// =====================================================================
//  🔄 NORMAL FUNCTION BHI CLOSURE KI JAGAH CHAL JAATA HAI!
// =====================================================================
//
//  KAHANI:
//  -------
//  Pichli file mein humne Manager ko ek CLOSURE diya tha.
//  Ab twist ye hai:
//     "Bhai, jaha Rust CLOSURE maang raha hai (Fn/FnMut/FnOnce),
//      waha tu NORMAL FUNCTION bhi de sakta hai!"
//
//  Kyun? Kyunki:
//     Normal function kuch bhi CAPTURE nahi karta (bahar se kuch nahi leta).
//     Jo kuch capture nahi karta, wo sabse SHAREEF hai.
//     Shareef hone ki wajah se wo Fn, FnMut, AUR FnOnce — TEENO implement karta hai!
//
//  Soch aise:
//     Closure = Employee jo office ka saamaan use karta hai (capture)
//     Function = Freelancer jo apna sab kuch laata hai (no capture)
//     Freelancer ko kahi bhi bitha do, koi dikkat nahi! 🎯
//
// =====================================================================

// Same Manager hai jo pichli file mein tha — "3 baar kaam chalaunga"
fn execute_thrice<F>(mut procedure: F)
where
    F: FnMut(),
{
    println!("--- Recruiter is hiring a task ---");
    procedure(); // 1st baar
    procedure(); // 2nd baar
    procedure(); // 3rd baar
}

// ┌──────────────────────────────────────────────────────────────┐
// │  YE EK NORMAL FUNCTION HAI — koi closure nahi               │
// │                                                              │
// │  Ye BAHAR se kuch nahi leta (no capture).                    │
// │  Isliye Rust isko automatically Fn + FnMut + FnOnce          │
// │  TEENO maan leta hai.                                        │
// │                                                              │
// │  Manager ne FnMut maanga? Koi baat nahi,                     │
// │  ye function FnMut bhi hai! Seedha bhej do.                  │
// └──────────────────────────────────────────────────────────────┘
fn bake_cake() {
    println!("🎂 Robot baking a simple cake...");
}

fn main() {
    // ── EXAMPLE 1: Normal function ko closure ki jagah bhej diya ──
    //
    //  execute_thrice CLOSURE maang raha hai (FnMut).
    //  Par humne NORMAL FUNCTION `bake_cake` de diya.
    //  Aur Rust ne kuch nahi bola! Kyunki function = sabse shareef closure.
    //
    //  Dhyaan se dekh:
    //    execute_thrice(bake_cake)    ← function ka NAAM diya, () nahi lagaya!
    //                   ^^^^^^^^^
    //    Agar bake_cake() likhte to function CALL ho jata.
    //    Bina () ke likha to function ka "pointer" gaya — matlab
    //    "ye raha mera kaam, jab chahiye tab chalaa lena."
    //
    execute_thrice(bake_cake);

    // ── EXAMPLE 2: Real-world — unwrap_or_else mein function bheja ──
    //
    //  option = None hai (kuch nahi hai andar).
    //  unwrap_or_else kehta hai:
    //    "Value nahi hai? To mujhe ek BACKUP PLAN de (closure ya function)
    //     jo value bana ke de."
    //
    //  Hum Vec::new de rahe hain — ye ek FUNCTION hai jo khaali Vec banata hai.
    //  Closure se likhte to aisa hota: || Vec::new()
    //  Par seedha function dena CHHOTA aur SAAF hai!
    //
    let option: Option<Vec<String>> = None;

    let collection = option.unwrap_or_else(Vec::new);
    //                                     ^^^^^^^^
    //                   Closure nahi hai! Seedha function diya. Clean! ✅

    println!("Final collection: {:?}", collection);
}

/*
EXPECTED OUTPUT:
--- Recruiter is hiring a task ---
🎂 Robot baking a simple cake...
🎂 Robot baking a simple cake...
🎂 Robot baking a simple cake...
Final collection: []
*/

/*
=====================================================================
  🎬 POORA EXECUTION FLOW

  main() shuru hua
       │
       │
  ═══ EXAMPLE 1: Normal Function as Closure ═══
       │
       ▼
  execute_thrice(bake_cake)  ← function ka pointer bheja, call nahi kiya
       │
       ├──► Manager ke paas `bake_cake` function aaya
       │
       ├──► procedure()  ──► 1st baar ──► "🎂 Robot baking a simple cake..."
       │
       ├──► procedure()  ──► 2nd baar ──► "🎂 Robot baking a simple cake..."
       │
       ├──► procedure()  ──► 3rd baar ──► "🎂 Robot baking a simple cake..."
       │
       │
  ═══ EXAMPLE 2: unwrap_or_else ═══
       │
       ▼
  option = None  (kuch nahi hai andar)
       │
       ▼
  option.unwrap_or_else(Vec::new)
       │
       ├──► Rust ne dekha: "Value hai? → NAHI (None)"
       │
       ├──► "Theek hai, backup plan chalata hoon → Vec::new()"
       │
       ├──► Khaali Vec ban gaya: []
       │
       ▼
  collection = []
  Print: "Final collection: []"

=====================================================================

  🤔 TO CLOSURE AUR FUNCTION MEIN FARAK KYA HAI?

  ┌──────────────────────┬───────────────────────────────────┐
  │     CLOSURE          │     NORMAL FUNCTION               │
  ├──────────────────────┼───────────────────────────────────┤
  │ Bahar ki cheezein    │ Bahar se kuch nahi leta           │
  │ CAPTURE kar sakta    │ (No capture)                      │
  │                      │                                   │
  │ || { ... }           │ fn naam() { ... }                 │
  │                      │                                   │
  │ Flexible hai         │ Sabse SHAREEF hai                 │
  │ (environment use     │ (Fn + FnMut + FnOnce              │
  │  karta hai)          │  TEENO implement karta hai)       │
  └──────────────────────┴───────────────────────────────────┘

  DONO ko waha bhej sakte ho jaha Fn/FnMut/FnOnce maanga ho!

  Closure likhna:  execute_thrice(|| println!("hi"))
  Function dena:   execute_thrice(bake_cake)

  Jab tujhe bahar ka kuch CAPTURE nahi karna,
  to normal function de — SAAF aur CHHOTA code! 🎯

=====================================================================

  🔑 PICHLI FILES SE CONNECT:

  File 1 (Tijori):     Closure diya ──► FnOnce (ek baar chala)
  File 2 (Naksha):     Closure diya ──► FnMut  (baar baar chala)
  File 3 (Manager):    Closure diya ──► FnMut  (3 baar chala)
  YE File:             FUNCTION diya ─► FnMut  (3 baar chala, BINA closure ke!)

  Matlab: Function = "Portable Closure" jisko kahi bhi bhej sakte ho! 🎯
=====================================================================
*/
