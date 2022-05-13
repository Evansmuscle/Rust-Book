pub fn variables_and_mutability() {
    // Mutable and Immutable variables
    let x = 10;
    let mut y = 20;
    println!("Mutable but immutated variable: {}", y);

    y = 30;
    println!("Immutable variable: {}, Mutated variable: {}", x, y);
    // Constants
    const ENVIRONMENT: &str = "PRODUCTION";
    println!("{}", ENVIRONMENT);
    // Shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);
}
