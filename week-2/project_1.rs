fn main() {
	let p: f64 = 520_000.0; // Principal
	let r: f64 = 10.0;       // Rate in percent
	let n: f64 = 5.0;         // Number of years

	// Calculate total amount (A)
	let a = p * (1.0 + (r / 100.0)).powf(n);

	// Calculate compound interest (CI)
	let ci = a - p;

	println!("Total Amount after {} years is: N{}", n,a);
	println!("Compound Interest is: N{}", ci);
}