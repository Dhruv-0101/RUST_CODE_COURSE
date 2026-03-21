fn main() {
    // 1. Statements (Instructions that perform some action but don't return a value)
    // Someone telling you: "Go pick up that glass." (Action performed, but no 'result' handed back)
    let y = 6; // This is a statement. It ends with a semicolon.

    // In Rust, you CANNOT do: let x = (let y = 6);
    // Because 'let y = 6' doesn't evaluate to anything. It has no value.

    // ---------------------------------------------------------

    // 2. Expressions (They evaluate to a value)
    // Someone asking you: "What is 5 + 6?" (You calculate it and give back '11')
    let x = 5 + 6; // '5 + 6' is an expression that evaluates to 11.

    // Expressions can be part of statements.
    // Here, 'x + 1' is the expression, and 'let result = ...;' makes it a statement.
    let result = x + 1;

    println!("The value of x is: {x}");
    println!("The value of result is: {result}");

    // ---------------------------------------------------------

    // 3. The Power of "Scope Blocks" as Expressions
    // This is where Rust feels different. A block {} is an expression!

    let z = {
        let a = 3;
        a + 1 // <--- NO SEMICOLON! This makes it an expression.
    };

    // How to 'feel' the block expression:
    // Imagine the block is a small factory.
    // Inside, it does some work (let a = 3).
    // The very last thing it "spits out" is the value 'a + 1' (which is 4).
    // Because there is no semicolon, that value is handed over to 'z'.

    println!("The value of z is: {z}");

    // ---------------------------------------------------------

    // 4. What happens if you add a Semicolon?

    let w = {
        let b = 10;
        b + 5; // <--- SEMICOLON ADDED! This is now a Statement.
    };

    // Because we added a semicolon, the block doesn't "spit out" 15.
    // Instead, it returns () (the 'Unit' type, which means empty).
    // 'w' now holds nothingness.
    println!("The value of w is: {:?}", w);
}

// Summary:
// - Statements: End with ; (They DO things).
// - Expressions: No ; at the end (They ARE things / they have value).
// - Turning an expression into a statement: Add a ;
/*
let z = {
    let a = 3;  // Semicolon hai -> Statement
    a + 1       // Semicolon NAHI hai -> Expression!
};

*/
