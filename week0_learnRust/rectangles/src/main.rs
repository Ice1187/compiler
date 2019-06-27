#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}

	fn square(size: u32) -> Rectangle {
		Rectangle { width: size, height: size}
	}
}

fn main() {
	let rect1 = Rectangle { width:30 , height:50 };
	let rect2 = Rectangle { width:10 , height:40 };
	let rect3 = Rectangle { width:60 , height:45 };

	println!("The area of the rect1 is {} pixels.", rect1.area());
	
	println!(" Can ret1 hold rect2? {}", rect1.can_hold(&rect2));
	println!(" Can ret2 hold rect3? {}", rect2.can_hold(&rect3));
	println!(" Can ret3 hold rect1? {}", rect3.can_hold(&rect1));

	let rect4 = Rectangle::square(20);

	println!("The square rectangle is {:?}", rect4);
} 

