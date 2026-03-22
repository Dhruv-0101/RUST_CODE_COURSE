// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 17)
// ==============================

// Topic: Multiple Lifetime Return (Deep Precision)
// Feel: 
// "Bhai, jab hamara return value sirf pehle parameter (`'a`) se hi connected hai, 
// toh doosre parameter (`'b`) ke scope se hume parwah nahi. 
// Doosra parameter bhale hi inner scope mein kyu na ho, result hamesha valid rahega!"

fn longest<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
    // Feel: Hum 'first' return karenge, par second ko sirf access 
    // karenge (e.g. printing ke liye). 
    println!("Printing second for fun: {}", second);
    first 
}

fn main() {
    let orlando = String::from("Orlando");
    let result;

    {
        // 'san_francisco' scope ke andar hai.
        let san_francisco = String::from("San Francisco");
        
        // result only depends on 'orlando' lifetime ('a).
        // san_francisco ('b') can die without affecting result!
        result = longest(&orlando, &san_francisco);
        println!("Inside Result: {}", result);
    } 

    // Orlando is still alive, so result is safe! 🎯
    println!("Outside Result (Orlando still exists): {}", result);
}

/*
EXPECTED OUTPUT:
Printing second for fun: San Francisco
Inside Result: Orlando
Outside Result (Orlando still exists): Orlando
*/

/*
🧠 Deep Feel:
Agar hum function ko `longest<'a>(first: &'a str, second: &'a str) -> &'a str` likhte, 
toh result inner scope ke baad INVALID ho jata. 
Kyunki `'a` donor members (Orlando & San Francisco) mein se SMALLER wali pick karta. 
Lekin `'a` aur `'b` ke saath, humne dono ko ALAG rakha. 
Since return type `&'a str` hai, wo sirf 'Orlando' ke zinda rehne se khush hai. 
Ab aaya pure precision wala life cycle feel! 🦀🚀
*/
