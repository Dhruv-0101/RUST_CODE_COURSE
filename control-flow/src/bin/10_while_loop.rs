fn main() {
    let mut seconds = 21;

    while seconds > 0 {
        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds...");
            seconds -= 3;
            continue;
        }

        println!("{seconds} seconds to blastoff...");
        seconds -= 1;
    }

    println!("Blastoff! 🚀");
}
/*
🔹 BREAK = “RUKO” / “Yahan se bahar nikal jao”

Meaning: Loop ko turant band kar deta hai.
➡️ Jahan break likha, loop wahi khatam.

Example feeling:
Jaise teacher bole — “Bas! Aage mat padhna, class khatam.”

🔹 CONTINUE = “CHHOD DO” / “Isko skip karo aur aage badho”

Meaning: Is iteration ko skip karke next iteration start.
➡️ Loop rukta nahi, sirf current step ko chhod ke aage chalta.

Example feeling:
Jaise teacher bole — “Is question ko skip karo, next question karo.”
*/