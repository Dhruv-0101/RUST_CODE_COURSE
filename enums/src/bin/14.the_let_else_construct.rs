#[derive(Debug)]
enum Milk {
    Whole,
    Lowfat(i32),
    NonDairy { kind: String },
}

fn main() {
    let my_beverage = Milk::NonDairy {
        kind: String::from("Oat"),
    };

    // 🎯 let-else: The "VIP Pass" (Kick them out or let them in for good!)
    let Milk::NonDairy { kind } = &my_beverage else {
        // This block MUST "diverge" (stop the function/loop)
        println!("You do not have the nondairy milk");
        return;
    };

    // ✅ SHAKTIMAAN MOMENT: 'kind' is now available in the WHOLE of main()!
    println!("{kind} milk is available here");

    /*
    =========================================================
    ❓ IF LET vs. LET ELSE: Local Room vs. Hallway
    =========================================================
    1. IF LET (Small Room Style):
       - In 'if let', the variable only lives inside the curly braces { ... }.
       - Outside those braces, the variable is GONE.
       - It's like: "You can enter this small room, but you can't come out."

    2. LET ELSE (Hallway Style):
       - In 'let else', the variable is defined for the REST of the function.
       - It's much cleaner if you want to use the variable for several lines
         later on, without nesting everything inside deep brackets.
       - It's like: "Show your ID. If you have it, you can walk anywhere in the building."

    =========================================================
    🚨 THE RULE: You MUST Diverge!
    =========================================================
    The 'else' block in 'let else' is NOT for printing things only.
    It's a "Guard". You MUST stop the code there using:
    - return;
    - break;
    - continue;
    - panic!();

    =========================================================
    SUMMARY:
    - if let: "If you match, go inside this scope."
    - let else: "If you DON'T match, GET OUT! If you do, stay in the main scope."
    =========================================================
    */
}

/*
    // 1️⃣ Pattern Match: Check if it's NonDairy (The Big Filter)
    let Milk::NonDairy { kind } = &my_beverage else {
        println!("You do not have the nondairy milk");
        return;
    };

    // 2️⃣ Value Check: Check if it's 'Oat' (The Specific Filter)
    if kind != "Oat" {
        println!("Wait! I wanted Oat, but this is {kind}.");
        return;
    }

    // ✅ SUCCESS: Now we are 100% sure it's the right choice!
    println!("{kind} milk is available here. Perfect!");

*/
