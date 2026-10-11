use std::io;

fn main() {
    println!("==================================================");
    println!("       PAN-ATLANTIC UNIVERSITY SHAPE CALCULATOR   ");
    println!("==================================================");
    println!("Select a shape option to compute:");
    println!("1. Area of a Trapezium");
    println!("2. Area of a Rhombus");
    println!("3. Area of a Parallelogram");
    println!("4. Surface Area of a Cube");
    println!("5. Volume of a Cylinder");
    println!("==================================================");

    print!("Enter choice (1-5): ");
    // Flush stdout to ensure the prompt displays before reading input
    let _ = io::Write::flush(&mut io::stdout());

    let mut choice_input = String::new();
    io::stdin()
        .read_line(&mut choice_input)
        .expect("Failed to read input");
    
    let choice: u32 = match choice_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid selection! Please enter a number between 1 and 5.");
            return;
        }
    };

    match choice {
        1 => calculate_trapezium(),
        2 => calculate_rhombus(),
        3 => calculate_parallelogram(),
        4 => calculate_cube(),
        5 => calculate_cylinder(),
        _ => println!("Error: Selected option is out of range. Please choose 1 to 5."),
    }
}

/// Helper function to read a floating point value safely from the console
fn read_f64(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input line");
        
        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid number format! Please try entering a valid value."),
        }
    }
}

/// 1. Trapezium Area = height / 2 * (base1 + base2)
fn calculate_trapezium() {
    println!("\n--- Area of a Trapezium Calculation ---");
    let height = read_f64("Enter the height:");
    let base1 = read_f64("Enter the length of base 1:");
    let base2 = read_f64("Enter the length of base 2:");

    let area = (height / 2.0) * (base1 + base2);
    println!("\nCalculated Area of the Trapezium: {:.4}", area);
}

/// 2. Rhombus Area = 0.5 * diagonal1 * diagonal2
fn calculate_rhombus() {
    println!("\n--- Area of a Rhombus Calculation ---");
    let diagonal1 = read_f64("Enter the length of diagonal 1:");
    let diagonal2 = read_f64("Enter the length of diagonal 2:");

    let area = 0.5 * diagonal1 * diagonal2;
    println!("\nCalculated Area of the Rhombus: {:.4}", area);
}

/// 3. Parallelogram Area = base * altitude
fn calculate_parallelogram() {
    println!("\n--- Area of a Parallelogram Calculation ---");
    let base = read_f64("Enter the base length:");
    let altitude = read_f64("Enter the altitude/height:");

    let area = base * altitude;
    println!("\nCalculated Area of the Parallelogram: {:.4}", area);
}

/// 4. Cube Surface Area = 6 * side * side
fn calculate_cube() {
    println!("\n--- Surface Area of a Cube Calculation ---");
    let side = read_f64("Enter the length of a side:");

    let surface_area = 6.0 * side * side;
    println!("\nCalculated Surface Area of the Cube: {:.4}", surface_area);
}

/// 5. Cylinder Volume = pi * radius * radius * height
fn calculate_cylinder() {
    println!("\n--- Volume of a Cylinder Calculation ---");
    let radius = read_f64("Enter the radius of the base:");
    let height = read_f64("Enter the height of the cylinder:");

    // Utilizing standard mathematical constant for PI
    let pi = std::f64::consts::PI; 
    let volume = pi * radius * radius * height;
    println!("\nCalculated Volume of the Cylinder: {:.4}", volume);
}
