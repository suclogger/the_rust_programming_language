fn main() {
	let mut s1 = String::from("hello");
	let r1 = &mut s1;
	let r2 = &mut s1;
	change(r1);
	println!("{}", s1);
	change(r2);
	println!("{}", s1);
}

fn change(s: &mut String) {
	s.push_str("world");
}
