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

// Program:
// 	_level = "Program"
// 	_type  = "FILE"
// 	_name  = file name

// Function:
// 	_level = "Function"
// 	_type  = TYPE_KEYWORD
// 	_name  = function name 

// Statement:
// 	_level = "Statement"
// 	_type  = STAT_KEYWORD
// 	_name  = stat name

// Expression:
// 	_level = "Expression"
// 	_type  = KEY_WORD
// 	_value = value