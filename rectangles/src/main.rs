fn main() {
	let rect1 = (30, 50);
	println!(
		"The area of the rectangle is {} square pixels",
		area(rect1)
	)
}

fn area(dimention: (u32, u32)) -> u32 {
	dimention.0*dimention.1
}
