mod inventory {
    const FLOOR_SPACE: u32 = 10000;
    pub const MANAGER: &str = "ivan inventory";

    #[derive(Debug)]
    enum ProductCategory {
        Ladder,
        Hammer,
    }

    #[derive(Debug)]
    struct Item {
        name: String,
        category: ProductCategory,
        quantity: u32,
    }

    fn talk_to_manage() {
        println!("Hey, {},how's your coffeee?", MANAGER);
    }
}

mod orders {
    pub const MANAGER: &str = "susan orders";
}

// ---------------------------------------------------------
// 🍔 THE RESTAURANT ANALOGY (Feel wali explanation)
// ---------------------------------------------------------
// "If a field is private, the struct must support 
// some associated function for creating an instance."
// ---------------------------------------------------------

mod restaurant {
    pub struct SecretBurger {
        // 🔒 PRIVATE FIELD: "Secret Sauce" ki recipe!
        // Ye field private hai, iska matlab koi bhi customer (bahar wala) 
        // kitchen ke andar jaake apna burger khud nahi assemble kar sakta.
        recipe: String, 
    }

    impl SecretBurger {
        // 👨‍🍳 ASSOCIATED FUNCTION (The Menu):
        // Chunki recipe private hai, aap burger sirf MENU se hi "Order" kar sakte ho!
        // Ye function 'new' hi wo rasta hai jo customer ko burger bana kar deta hai.
        pub fn order_from_menu() -> Self {
            Self {
                recipe: String::from("Magic Masala + Secret Sauce"), // Kitchen ke andar access hai!
            }
        }

        pub fn eat(&self) {
            println!("Wow! The burger with {} was delicious! 😋", self.recipe);
        }
    }
}

fn main() {
    // 🏢 AAPKA ORIGINAL CODE:
    println!("The manager of the inventory is {}", inventory::MANAGER);
    println!("The manager of the orders is {}", orders::MANAGER);

    /* 
    =========================================================
    🎯 FINAL FEEL (ASLI MATLAB):
    =========================================================
    1. Agar field private hai, toh uska 'Control' programmer ke paas hai.
    2. Associated function (like order_from_menu) wo "Official Gate" hai.
    3. Taaki koi bhi bahar wala kuch bhi galat-salat data (jaise 
       kharab tamatar) aapke struct mein na daal sake. 
    =========================================================
    */

    // ✅ Instance create karna (Ordering from the official menu):
    let my_burger = restaurant::SecretBurger::order_from_menu();
    my_burger.eat();
}
