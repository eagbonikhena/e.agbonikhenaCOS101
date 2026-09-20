// Rust program to find the roots of a quadratic equation
use std::io;

fn main() {
    println!("--- Quadratic Equation Solver ---");

    // Read input a
    println!("Enter coefficient a:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a: f32 = input1.trim().parse().expect("Invalid number");

    // Read input b
    println!("Enter coefficient b:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b: f32 = input2.trim().parse().expect("Invalid number");

    // Read input c
    println!("Enter coefficient c:");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c: f32 = input3.trim().parse().expect("Invalid number");

    // Calculate the discriminant[cite: 1]
    let d = (b * b) - (4.0 * a * c);

    // Decision-making based on discriminant values[cite: 1]
    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {} and {}", root1, root2);
    } else if d == 0.0 {
        let root = -b / (2.0 * a);
        println!("Exactly one real root: {}", root);
    } else {
        println!("No real roots.");
    }
}
