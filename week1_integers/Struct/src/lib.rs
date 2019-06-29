pub mod Token {
	pub struct Token {
		pub _type:  String,
		pub _value: String,
	}
	pub fn new() -> Token {
		Token {
			_type:  String::from("None type"),
			_value: String::from("None value")
		}
	}
}

pub mod Node {
	pub struct Node {
		pub _level: String,
		pub _type:  String,
		pub _name:  String,
		pub _value: String,
		pub to: Vec<usize>,
	}
	pub fn new() -> Node {
		Node {
			_level : "None _level".to_string(),
			_type  : "None type".to_string()  ,
			_name  : "None name".to_string()  ,
			_value : "None value".to_string() ,			
			to: Vec::new(),
		}
	}

}