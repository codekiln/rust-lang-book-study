/// "const can be used in the global scope, and let can only be used in a function""
fn test_const_vs_let() {
    const TWO: u32 = 1 + 1;
    let three = TWO + 1;
    println!("{TWO}");
    println!("{three}");
}

const FOUR: u32 = 2 + 2;

// causes compilation  - let can't be used in global scope
// let four = FOUR + 1;

// causes compilation err - macro 
// println!("{FOUR}");

fn test_mutable_variable() {
    let mut count = FOUR;
    println!("Initial count: {count}");
    count += 1;
    println!("Updated count: {count}");
}

fn test_shadowing() {
    let x = 5;
    println!("Initial x: {x}");
    let x = x + 1; // Shadowing
    // let x += 1; // causes compilation error
    println!("Shadowed x: {x}");
    {
        let x = x * 2; // Shadowing in inner scope
        println!("Inner scope x: {x}");
    }
    println!("Outer scope x: {x}");
}

fn test_postfix_increment() {
    let mut y = 10;
    println!("Initial y: {y}");
    // Rust does not support y++ or ++y
    y += 1; // Equivalent to y++
    println!("After increment y: {y}");
}

fn test_mutation() {
    let mut x = 20;
    println!("Initial x: {x}");
    {
        let mut x = x;
        x += 2;
    }
    println!("After mutation x: {x}");
}

fn test_float_conversion() {
    let x = 2.0;
    println!("Float x: {x}");
}

fn test_tuple_access() {
    let tup = (1, 2.0, 'a');
    let (a, b, c) = tup;
    println!("Tuple elements: {a}, {b}, {c}");
    // Tuple elements: 1, 2, a
}

fn main() {
    println!("Testing const vs let:");
    test_const_vs_let();
    println!("Testing global const:");
    println!("{FOUR}");
    println!("Testing mutable variable:");
    test_mutable_variable();
    println!("Testing shadowing:");
    test_shadowing();
    println!("Testing postfix increment:");
    test_postfix_increment();
    println!("Testing mutation:");
    test_mutation();
    println!("Testing float conversion:");
    test_float_conversion();
    println!("Testing tuple access:");
    test_tuple_access();
}