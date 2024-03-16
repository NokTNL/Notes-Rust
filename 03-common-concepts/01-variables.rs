fn main() {
    /**
     * Variables
     */
    // Variables are defined by `let`. They are IMMUTABLE by default.
    let x = 6;
    println!("x is: {x}"); // println! only accepts string literals, you need to use {} for dynamic values 
    // x = 5; // Reassgin will instant error at compilation!

    // Add `mut` to the variable to make it mutable. However, you can't reassign a different DATA TYPE
    let mut y = 7;
    println!("y is: {y}");
    // y = "a"; Compilation error if reassigned a string
    y = 10;
    println!("y is: {y}");

    // `const` variables are always immutable; you can't use `mut` on them
    // A data type is also necessary for const
    const DAYS_PER_YEAR: u32 = 365;
    println!("There are {DAYS_PER_YEAR} days in a year");

    // In summary, `let` variables CAN be mutable; `const` do not have that choice

    /**
     * Scope
     */
    // Rust variables are BRACKET-scoped. Simply wrapping variables inside a scope will limit 
    {
        let _scoped_var = 10; // <--- this is scoped to the outer bracket only
    }
    // println!("{_scoped_var}"); Compilation error because _scoped_var is out of scope

    /*
     * Shadowing
     */
    // A variable of the same name can be redeclared with `let` (but not `const`). The new variable will simply take over the old one.
    // Even data type can be overwritten when doing this (e.g. from a number to a string)
    let z = 2;
    let z = 3; // Legal; but the compiler will warn you that the first variable declaration is not used at all
    println!("z: {z}");
}