fn main() {
	let s1 = String::from("hello");
	change(&s1);
}

fn change(s: &String) {
	s.push_str(", world");
}
