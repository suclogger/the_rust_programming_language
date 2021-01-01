fn main() {
	let s = String::from("hello");
	takes_ownership(s);
	// this would produce error
	println!("{}", s);	
	let x = 5;
	makes_copy(x);
	// this is good to go
	println!("{}", x);
}

fn takes_ownership(some_string: String) {
	println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
	println!("{}", some_integer);
}
