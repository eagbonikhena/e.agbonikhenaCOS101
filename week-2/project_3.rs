fn main() {
    let p: f64 = 210_000.00; // Principal
    let r: f64 = 5.00; // Rate
    let n: f64 = 3.00; // Number of years

    // Formula: A = P * (1 - R / 100)^n
    let a = p * (1.0 - (r / 100.0)).powf(n);

    println!("Value of the TV after 3 years: ₦{}", a);
}