fn main () {
	let price_1: f64 = 450_000.00;
	let price_2: f64 = 1_500_000.00;
	let price_3: f64 = 750_000.00;
	let price_4: f64 = 2_850_000.00;
	let price_5: f64 = 250_000.00;

	let qty_1: f64 = 2.00;
	let qty_2: f64 = 1.00;
	let qty_3: f64 = 3.00;
	let qty_4: f64 = 3.00;
	let qty_5: f64 = 1.00;

	let price_sum = (price_1 * qty_1) + (price_2 * qty_2) + (price_3 * qty_3) + (price_4 * qty_4) + (price_5 * qty_5);

	let quantity_sum = qty_1 + qty_2 + qty_3 + qty_4 + qty_5 ;

	let average = price_sum / quantity_sum ;

	println!("Sum of total items is ₦{}", price_sum );
	println!("Average sale per item is ₦{}", average);
}