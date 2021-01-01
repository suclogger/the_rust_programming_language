fn main() {
	let mut s1 = String::from("hello");
	change(&mut s1);
	println!("{}", s1);
}

fn change(s: &mut String) {
	s.push_str(", world");
}
