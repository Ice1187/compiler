mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
	hosting::add_to_waithlist();
	hosting::add_to_waithlist();
	hosting::add_to_waithlist();
}

// mod back_of_house {
// 	pub struct Breakfast {
// 		pub toast: String,
// 		seasonal_fruit: String, 
// 	}

// 	impl Breakfast {
// 		pub fn summer(toast: &str) -> Breakfast {
// 			Breakfast {	
// 				toast: String::from(toast),
// 				seasonal_fruit: String::from("peaches"),
// 			}
// 		}
// 	}
// }

// pub fn eat_at_restaurant() {
// 	let mut meal = back_of_house::Breakfast::summer("Rye");

// 	println!("I'd like {} toast, please.", meal.toast);

// 	meal.toast = String::from("Wheat");

// 	println!("I'd like {} toast, please.", meal.toast);
// }

// fn serve_order() {};

// mod back_of_house() {
// 	fn fix_incorrect_order() {
// 		cook_order();
// 		super::serve_order();
// 	}

// 	fn cook_order() {}
// }




