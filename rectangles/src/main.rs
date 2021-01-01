#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width*self.height
	}
}

impl Rectangle {
	fn can_hold(&self, other_rect: &Rectangle) -> bool {
		other_rect.width < self.width && other_rect.height < self.width
	}
}

fn main() {
	let rect1 = Rectangle{width : 30, height: 50};
	let rect2 = Rectangle{width : 10, height: 20};
	let rect3 = Rectangle{width : 40, height: 60};
	println!("Can hold rect2? {}", rect1.can_hold(&rect2));
	println!("Can hold rect3? {}", rect1.can_hold(&rect3));
}
