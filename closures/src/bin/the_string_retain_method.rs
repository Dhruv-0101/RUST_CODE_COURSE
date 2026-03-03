// ==============================
// RUST CLOSURES — FULL FEEL VERSION (EPISODE 10)
// ==============================

// Topic: The retain Method for Strings
// Feel: 
// "Bhai, socho 'String' ek VIP Club hai.
// .retain() method ek Security Checkpoint hai. 
// Ye har character ko check karta hai aur decision leta hai: 
// Kaun andar rahega aur kaun bahar?"

fn main() {
    // 1. Hamara VIP Club "PlayStation" jisne bohot saari bheed (characters) hai.
    let mut game_console = String::from("PlayStation");
    
    // 2. Ek kachre ka dibba taiyar rakha (Kicked out characters ka record)
    let mut kicked_out = String::new();

    // ------------------------------
    // THE BOUNCER (The Closure) �‍♂️
    // ------------------------------
    // Feel: .retain() ke andar jo closure hai, wo ek 'Bouncer' hai.
    // Uska kaam hai: character ko dekhna aur 'true'/'false' bolna.
    let bouncer = |guest: char| {
        // Shart (The Rule): "Bhai, 'a' naam ke charcter ko andar entry nahi milegi!"
        if guest != 'a' {
            // Feel: True matlab "Bhai, tu andar reh sakta hai."
            true 
        } else {
            // Feel: False matlab "Nikal ja club se!"
            // Par nikaalne se pehle Bouncer ne use record book mein likh liya.
            kicked_out.push(guest);
            false 
        }
    };

    // 🚀 EXECUTION: 
    // .retain() bouncer ko bulata hai aur kehta hai: 
    // "Ek ek karke sabko check karo aur jo 'false' bole, use hata do."
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
🧠 Deep Feel (Aha! Moments):
1. Why Boolean? .retain() ko sirf ye chahiye ki 'Isko rakhu ya nahi?'. 
   True = Yes (Retain), False = No (Remove).
2. FnMut Magic: Bouncer closure `FnMut` implement kar raha hai 
   kyunki wo `kicked_out` (bahar ki memory) ko change kar raha hai. 
3. Efficiency: .retain() string ko wahin ki wahin (In-place) edit karta hai. 
   Naya string banane ka kharcha nahi hota!

Bhai, ye closure ka use-case hai jaha aap logic bhejte ho, 
data wahi collect ho raha hai! 🦾✨
*/
