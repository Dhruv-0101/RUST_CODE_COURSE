/* 
=========================================================
🚠 THE CRATE PREFIX: "The Skyscraper Teleport" (Ground Floor)
=========================================================
Socho aapka poora Rust project ek bahut badi "Skyscraper" (Crate) hai.

- Aap kisi bhi floor/room (module) mein ho sakte ho.
- 'self::' = Aap isi room mein dekh rahe ho (Relative).
- 'super::' = Aap ek floor upar (Parent floor) jaa rahe ho.

- 'crate::' = YE TELEPORT HAI! 🚠
  Aap chahe top floor ke kisi kone (deep module) mein ho, 
  'crate::' bolte hi aap seedha GROUND FLOOR (Reception) par 
  pahunch jaate ho (src/main.rs ya src/lib.rs).
  Wahan se aap kisi bhi floor ki lift le sakte ho!
=========================================================
*/

// Ye function dikhayega ki kaise hum 'crate' se kahin bhi jaa sakte hain
pub fn feel_the_teleport() {
    println!("\n🚠 Teleportation Active: Going to Ground Floor reception (crate root)...");

    // 🗺️ crate:: karke hum Ground Floor se start kar rahe hain:
    println!("Found Inventory Manager via Ground Floor: {}", crate::inventory::MANAGER);
    println!("Found Orders Manager via Ground Floor: {}", crate::orders::MANAGER);
    
    // Yahan hum 'crate::inventory::products' tak bhi jaa sakte hain:
    let item = crate::inventory::products::ProductCategory::Ladder;
    println!("Found ProductCategory: {:?}", item);
}
