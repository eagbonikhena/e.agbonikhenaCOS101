use std::io;

fn main() {
    println!("=== Public Service APS Level Checker ===");

    // Define table data using vectors: (Position, Min Years, Max Years, APS Level)
    let levels: Vec<(&str, u32, u32, &str)> = vec![
        ("Public Servant", 0, 0, "APS 1-2"),
        ("Public Servant", 1, 4, "APS 3-5"),
        ("Public Servant", 5, 8, "APS 5-8"),
        ("Public Servant", 8, 10, "EL1 8-10"),
        ("Public Servant", 10, 13, "EL2 10-13"),
        ("Public Servant", 14, 100, "SES"),

        ("Office Admin", 0, 0, "Intern-Paralegal"),
        ("Office Admin", 1, 4, "Administrator"),
        ("Office Admin", 5, 8, "Senior Administrator"),
        ("Office Admin", 8, 10, "Office Manager"),
        ("Office Admin", 10, 13, "Director"),
        ("Office Admin", 14, 100, "CEO"),

        ("Academic", 0, 0, "Placement"),
        ("Academic", 1, 4, "Research Assistant"),
        ("Academic", 5, 8, "PhD Candidate"),
        ("Academic", 8, 10, "Post-Doc Researcher"),
        ("Academic", 10, 13, "Senior Lecturer"),
        ("Academic", 14, 100, "Dean"),

        ("Lawyer", 0, 0, "Placement"),
        ("Lawyer", 1, 4, "Junior Associate"),
        ("Lawyer", 5, 8, "Associate"),
        ("Lawyer", 8, 10, "Senior Associate 1-2"),
        ("Lawyer", 10, 13, "Senior Associate 3-4"),
        ("Lawyer", 14, 100, "Partner"),

        ("Teacher", 0, 0, "Placement"),
        ("Teacher", 1, 4, "Classroom Teacher"),
        ("Teacher", 5, 8, "Senior Teacher"),
        ("Teacher", 8, 10, "Leading Teacher"),
        ("Teacher", 10, 13, "Deputy Principal"),
        ("Teacher", 14, 100, "Principal"),
    ];

    // Read Job Category
    println!("\nEnter your Job Category (Public Servant, Office Admin, Academic, Lawyer, Teacher):");
    let mut category_input = String::new();
    io::stdin().read_line(&mut category_input).expect("Failed to read line");
    let category = category_input.trim();

    // Read Years of Experience
    println!("Enter your Years of Experience:");
    let mut exp_input = String::new();
    io::stdin().read_line(&mut exp_input).expect("Failed to read line");
    let years: u32 = exp_input.trim().parse().unwrap_or(0);

    // Find and output the matching position level
    let mut found = false;
    for entry in &levels {
        let (pos, min_yrs, max_yrs, title) = entry;
        if category.eq_ignore_ascii_case(pos) && years >= *min_yrs && years <= *max_yrs {
            println!("\n>>> Result: Your position title/level is: {} <<<", title);
            found = true;
            break;
        }
    }

    if !found {
        println!("\nInvalid input or no matching APS level found for the given criteria.");
    }
}