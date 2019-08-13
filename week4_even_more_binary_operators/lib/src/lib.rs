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
			_level : String::from("None _level"),
			_type  : String::from("None type")  ,
			_name  : String::from("None name")  ,
			_value : String::from("None value") ,			
			to: Vec::new(),
		}
	}

}


pub mod Pprint {

	use std::io::{stdout, Write};
	pub fn print_token(token_vec: & Vec<crate::Token::Token>) {
		let c = token_vec.len();
		print!{"\n[-] Token: ["};
		for i in 0..c {
	    	print!("\"{}: {}\", ", token_vec[i]._type, token_vec[i]._value);
			stdout().flush().unwrap();
	    }
	    print!("]\n");
		stdout().flush().unwrap();
	}

	pub fn print_ast(ast: & Vec<crate::Node::Node>, node: usize) {
		if node == 0 {
			println!{"\n[-] AST: "};
		}
		if ast[node]._level == "Program".to_string() {
	    	println!("{} {}: {} -> {:?}", node, ast[node]._level, ast[node]._name, ast[node].to);
		} else if ast[node]._level == "Function".to_string() {
			print!("  ");		
	    	println!("{} {}: {} -> {:?}", node, ast[node]._level, ast[node]._name, ast[node].to);
		} else if ast[node]._level == "Statement".to_string() {
			print!("    ");
    		println!("{} {}: {} -> {:?}", node, ast[node]._level, ast[node]._name, ast[node].to);
		} else if ast[node]._level == "Expression".to_string() {
			print!("      ");
	    	println!("{} {}: {} {} -> {:?}", node, ast[node]._level, ast[node]._type, ast[node]._value, ast[node].to);
		} else { println!("[!] Unrecognize ast: {} {} {} {}", ast[node]._level, ast[node]._type, ast[node]._name, ast[node]._value);}
		for to in ast[node].to.iter() {
	    	print_ast(& ast, *to);
		}
	}

	use std::fs::File;
	use std::io::{BufRead, BufReader};
	pub fn print_asm(filename: & String) {
	    let file = File::open(&filename).expect("\nP asm: Failed to open the file\n");
		let reader = BufReader::new(file);
		println!("\n[-] Assembly:");
		for line in reader.lines() {
			println!("{}", line.expect("P asm: Failed to read the lines"));
		}
	}

	pub fn print_file(filename: & String) {
	    let file = File::open(&filename).expect("\nP file: Failed to open the file\n");
		let reader = BufReader::new(file);
		println!("\n[-] File: {}", &filename);
		for line in reader.lines() {
			println!("{}", line.expect("P file: Failed to read the lines"));
		}
	}


}
