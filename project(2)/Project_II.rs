use std::io;

fn main() {
    println!("=== EY Nigeria Developer Hiring Portal ===");

    // Vector to hold candidate data as tuples: (Name, Years of Experience)
    let mut candidates: Vec<(String, u32)> = Vec::new();

    // Input number of candidates
    let mut num_input = String::new();
    println!("\nEnter the number of interview candidates:");
    io::stdin().read_line(&mut num_input).expect("Failed to read input");
    let count: u32 = num_input.trim().parse().unwrap_or(0);

    // Collect candidate details
    for i in 0..count {
        println!("\nCandidate {}:", i + 1);

        let mut name = String::new();
        println!("Enter name:");
        io::stdin().read_line(&mut name).expect("Failed to read name");

        let mut years_str = String::new();
        println!("Enter years of programming experience:");
        io::stdin().read_line(&mut years_str).expect("Failed to read experience");
        let years: u32 = years_str.trim().parse().unwrap_or(0);

        candidates.push((name.trim().to_string(), years));
    }

    // Check if candidates list is empty
    if candidates.is_empty() {
        println!("\nNo candidates were entered.");
        return;
    }

    // Loop through the vector of tuples to find the highest experience
    let mut max_candidate = &candidates[0];

    for candidate in &candidates {
        if candidate.1 > max_candidate.1 {
            max_candidate = candidate;
        }
    }

    // Display the winner
    println!("\n==========================================");
    println!("   EY NIGERIA MOST EXPERIENCED CANDIDATE  ");
    println!("==========================================");
    println!("Name: {}", max_candidate.0);
    println!("Years of Experience: {} years", max_candidate.1);
}