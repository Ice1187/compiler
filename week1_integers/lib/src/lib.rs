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


pub mod Pprint {
	pub fn print_ast(ast: & Vec<crate::Node::Node>, verbose: i32) {
		let c = ast.len();
		println!{"\n[-] AST: "};
		if verbose == 1 {
			for i in 0..c {
				if ast[i]._level == "Program".to_string() {
			    	println!("{} {}: {:?}", i, ast[i]._level, ast[i].to);
				} else if ast[i]._level == "Function".to_string() {
					 print!("  ");
			    	println!("{} {}: {:?}", i, ast[i]._level, ast[i].to);
				} else if ast[i]._level == "Statement".to_string() {
					 print!("    ");
			    	println!("{} {}: {:?}", i, ast[i]._level, ast[i].to);
				} else if ast[i]._level == "Expression".to_string() {
					 print!("      ");
			    	println!("{} {}: {:?}", i, ast[i]._level, ast[i].to);
				} else { println!("No Expression?");}
		    }			
		}
		else if verbose == 2 {
			for i in 0..c {
				if ast[i]._level == "Program".to_string() {
			    	println!("{} {}: {} -> {:?}", i, ast[i]._level, ast[i]._name, ast[i].to);
				} else if ast[i]._level == "Function".to_string() {
					 print!("  ");
			    	println!("{} {}: {} {} -> {:?}", i, ast[i]._level, ast[i]._type, ast[i]._name, ast[i].to);
				} else if ast[i]._level == "Statement".to_string() {
					 print!("    ");
			    	println!("{} {}: {} -> {:?}", i, ast[i]._level, ast[i]._name, ast[i].to);
				} else if ast[i]._level == "Expression".to_string() {
					 print!("      ");
			    	println!("{} {}: {} {} -> {:?}", i, ast[i]._level, ast[i]._type, ast[i]._value, ast[i].to);
				} else { println!("No Expression?");}
		    }	
		}
		println!("");
	}

	use std::io::{stdout, Write};
	
	pub fn print_token(token_vec: & Vec<crate::Token::Token>) {
		let c = token_vec.len();
		print!{"[-] Token: ["};
		for i in 0..c {
	    	print!("\"{}: {}\", ", token_vec[i]._type, token_vec[i]._value);
			stdout().flush().unwrap();
	    }
	    print!("]\n");
		stdout().flush().unwrap();
	}

	use std::fs::File;
	use std::io::{BufRead, BufReader};

	pub fn print_asm(filename: & String) {
	    let file = File::open(&filename).expect("\nP asm: Failed to open the file\n");
		let reader = BufReader::new(file);
		for line in reader.lines() {
			println!("{}", line.expect("P asm: Failed to read the lines"));
		}
	}
}
