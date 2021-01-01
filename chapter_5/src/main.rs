struct User{
	name: &str,
	email: &str,
	sign_in_count: u64,
	active: bool,
}

fn main() {
	let user1 = User{
		email: "pr0found@example.com",
		username: "pr0found",
		active: true,
		sigh_in_count: 1,
	};
}
