enum Milk {
    Lowfat(i32),
    Whole,
    NonDairy { kind: String },
}

fn main() {
    let my_beverage = Milk::Whole;

    if let Milk::Whole = my_beverage {
        println!("You have the Whole milk");
    }

    let my_beverage = Milk::Lowfat(3);

    if let Milk::Lowfat(percent) = my_beverage {
        println!("Your beverage is {percent}% milk.");
    }

    let my_beverage = Milk::NonDairy {
        kind: String::from("oat"),
    };

    // ❓ WHY THE '&'? (The Borrow Mystery)
    if let Milk::NonDairy { kind } = &my_beverage {
        if kind == "oat" {
            println!("Wait! This is EXACTLY {kind} milk. My favorite!");
        }
    }
}

// =========================================================
// ❓ THE '&' MYSTERY: Camera vs. Thief
// =========================================================
/*
Aapne bahut solid sawal poocha! `&` yahan ownership ko
bachane ke liye lagaya gaya hai.

1. CASE A: If we write `if let ... = my_beverage` (THIEF Style) ❌
------------------------------------------------------------------
- Rust `kind` (String) ko `my_beverage` ke andar se **MOVE**
  kar lega.
- Iska matlab aapne glass ke andar se milk nikal ke dusre glass
  mein daal diya.
- Ab `my_beverage` "Khaali" (Invalid) ho gaya. Aap use niche
  dobara use nahi kar payenge.

2. CASE B: If we write `if let ... = &my_beverage` (CAMERA Style) ✅
--------------------------------------------------------------------
- Yahan hum poore object ka sirf ek **Reference (Borrow)** le rahe hain.
- `kind` ab ek asali String nahi, balki ek **Reference (&String)** hai.
- Ye bilkul aisa hai ki aapne milk ko piya nahi, balki sirf
  uski **PHOTOS** li.
- Photo lene se milk khatam nahi hota, isiliye `my_beverage`
  abhi bhi valid hai!

SUMMARY:
- Without `&`: Ownership moves out (Destruction).
- With `&`: We just peek inside (Borrowing).
- Use `&` whenever you want to keep using the original variable later!
*/

// =========================================================
// 🎯 THE "DOUBLE FILTER" ANALOGY
// =========================================================
/*
1. THE SCENE: "Finding the Right Item"
--------------------------------------
Sometimes just finding the 'type' (NonDairy) isn't enough.
You also want to check 'what kind' (oat, soy, etc.).

2. HOW IT WORKS:
----------------
- Step A: `if let Milk::NonDairy` -> "Is it Non-Dairy?"
- Step B: `if kind == "oat"` -> "Is it specifically OAT?"

It's like looking for a "Blue Pen":
- First: Check if it's a PEN (`if let Pen`).
- Second: Check if it's BLUE (`if color == "blue"`).

3. WHY NOT IN ONE LINE?
-----------------------
In Rust, `if let` handles the "Structure" of the enum.
For checking the "Content" (Values like String), it's
cleaner to add another `if` inside.

SUMMARY:
- Match the structure with `if let`.
- Match the value with a regular `if`.
- It's a "Filter inside a Filter"!
*/

// =========================================================
// 🔍 IF LET: The "Quick Filter"
// =========================================================

/*
1. THE PROBLEM: "Match is too noisy sometimes"
----------------------------------------------
Imagine you have 10 variants in an Enum, but you ONLY care
about ONE. Using `match` would look like this:

match my_milk {
    Milk::Whole => println!("Yay!"),
    _ => (),  // 🗑️ THE "CATCH-ALL" (Kyun likhte hain?)
}

/*
🤔 WHAT IS `_ => ()`? (Bhook nahi lagi!)
--------------------------------------
1. THE `_` (Underscore):
   - Ye ek "Placeholder" hai. Iska matlab: "Anything else!"
   - Rust kehta hai: "Bhai, saare cases handle karo."
     Toh hum kehte hain: "Baaki jo bacha (`_`), use pakad lo."

2. THE `=> ()` (Unit):
   - Iska matlab: "Kuch mat karo" (Do nothing).
   - `()` Rust mein 'Unit Type' hai, yani "khaali".

3. THE FEEL (The Trash Can):
   - Maan lijiye aapke pass ek bucket mein alag-alag types ki coins hain.
   - Aapko sirf 5 Rs ka coin chahiye.
   - Baki sab (1 Rs, 2 Rs, Stone) aap dustbin mein daal doge.
   - `_ => ()` wahi dustbin hai, jahan baaki sab "waste" jaa raha hai.
*/

2. THE SOLUTION: if let
-----------------------
It's like a focused `match`. It says:
"IF this value matches THIS specific pattern, then do the action.
Otherwise, just ignore it."

if let Milk::Whole = my_milk {
    println!("Yay!"); // ✅ CLEAN! No extra code needed.
}

3. THE VISUAL (Match vs. if let)
--------------------------------
   [ MATCH ]                            [ IF LET ]
   +-----------------------+            +-----------------------+
   |  Menu of Options:     |            |  Check one thing:     |
   |  1. Pizza   -> Eat    |            |                       |
   |  2. Burger  -> Eat    |            |  Pizza? -> Eat!       |
   |  3. Salad   -> Eat    |            |  (Ignore everything   |
   |  4. Others  -> Ignore |            |   else automatically) |
   +-----------------------+            +-----------------------+

4. PROS & CONS:
----------------
- PRO: Much cleaner code when you only have one case.
- CON: You lose the "Exhaustiveness Check" (Rust won't warn
  if you forgot a variant).

SUMMARY:
- `match` = A full menu (Handle everything).
- `if let` = A quick snack (Handle only what you want).
*/
