use std::io;

fn main() {
    // Display Menu
    println!("==========================================");
    println!("           THE RESTAURANT MENU            ");
    println!("==========================================");
    println!("P - Poundo Yam / Edinkaiko Soup : N3,200");
    println!("F - Fried Rice & Chicken        : N3,000");
    println!("A - Amala & Ewedu Soup          : N2,500");
    println!("E - Eba & Egusi Soup            : N2,000");
    println!("W - White Rice & Stew           : N2,500");
    println!("==========================================");

    // Read food type selection
    println!("Enter food code (P, F, A, E, W):");
    let mut food_code = String::new();
    io::stdin()
        .read_line(&mut food_code)
        .expect("Failed to read input");
    
    // Clean and normalize food code input
    let food_code = food_code.trim().to_uppercase();

    // Determine price per item based on food code
    let price: f64 = if food_code == "P" {
        3200.0
    } else if food_code == "F" {
        3000.0
    } else if food_code == "A" {
        2500.0
    } else if food_code == "E" {
        2000.0
    } else if food_code == "W" {
        2500.0
    } else {
        println!("Invalid food code selected!");
        return;
    };

    // Read quantity
    println!("Enter quantity:");
    let mut quantity_str = String::new();
    io::stdin()
        .read_line(&mut quantity_str)
        .expect("Failed to read input");

    let quantity: f64 = match quantity_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number for quantity!");
            return;
        }
    };

    // Calculate initial total
    let mut total_charge = price * quantity;
    println!("\nSubtotal: N{:.2}", total_charge);

    // Apply 5% discount if total is greater than N10,000
    if total_charge > 10000.0 {
        let discount = total_charge * 0.05;
        total_charge -= discount;
        println!("Discount Applied (5%): -N{:.2}", discount);
    }

    println!("Total Charge: N{:.2}", total_charge);
}