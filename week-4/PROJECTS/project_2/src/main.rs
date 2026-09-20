// Rust program to calculate employee annual incentive
use std::io;

fn main() {
    println!("--- Employee Incentive Calculator ---");

    // Input experience level
    println!("Is the employee experienced? (enter 'yes' or 'no'):");
    let mut exp_input = String::new();
    io::stdin().read_line(&mut exp_input).expect("Failed to read input");
    let is_experienced = exp_input.trim().to_lowercase() == "yes";

    // Input age
    println!("Enter age:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: u32 = age_input.trim().parse().expect("Invalid age input");

    // Determine incentive based on experience and age criteria[cite: 1]
    if is_experienced {
        if age >= 40 {
            println!("Annual Incentive: N1,560,000");
        } else if age >= 30 && age <= 39 {
            println!("Annual Incentive: N1,480,000");
        } else if age < 28 {
            println!("Annual Incentive: N1,300,000");
        } else {
            println!("No incentive tier specified for experienced employees between 28 and 29.");
        }
    } else {
        println!("Annual Incentive: N100,000");
    }
}