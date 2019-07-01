use std::io::{stdout, Write};
use struct_lib::Token;
use struct_lib::Node;
use lexer_lib;

fn set_token(token_vec: &mut Vec<Token::Token>) {
	token_vec.push(Token::Token {
		_type:  "INT_KEYWORD".to_string(),
		_value: "int".to_string(),
	} );
	token_vec.push(Token::Token {
		_type:  "IDENTIFIER".to_string(),
		_value: "main".to_string(),
	} );
	token_vec.push(Token::Token {
		_type:  "OPEN_PAREN".to_string(),
		_value: "(".to_string(),
	} );
	token_vec.push(Token::Token {
		_type:  "CLOSE_PAREN".to_string(),
		_value: ")".to_string(),
	} );
	token_vec.push(Token::Token {
		_type:  "OPEN_BRACE".to_string(),
		_value: "{".to_string(),
	} );
	token_vec.push(Token::Token {
		_type:  "RETURN_KEYWORD".to_string(),
		_value: "return".to_string(),
	} );
	token_vec.push(Token::Token {
		_type:  "CONSTANT".to_string(),
		_value: "2".to_string(),
	} );
	token_vec.push(Token::Token {
		_type:  "SEMICOLON".to_string(),
		_value: ";".to_string(),
	} );
	token_vec.push(Token::Token {
		_type:  "RETURN_KEYWORD".to_string(),
		_value: "return".to_string(),
	} );
	token_vec.push(Token::Token {
		_type:  "CONSTANT".to_string(),
		_value: "3".to_string(),
	} );
	token_vec.push(Token::Token {
		_type:  "SEMICOLON".to_string(),
		_value: ";".to_string(),
	} );
	token_vec.push(Token::Token {
		_type:  "CLOSE_BRACE".to_string(),
		_value: "}".to_string(),
	} );
}

fn print_token(token_vec: & Vec<Token::Token>) {
	let c = token_vec.len();
	print!{"[-] Token: ["};
	for i in 0..c {
    	print!("\"{}: {}\", ", token_vec[i]._type, token_vec[i]._value);
		stdout().flush().unwrap();
    }
    print!("]\n");
	stdout().flush().unwrap();
}

fn print_ast(ast: & Vec<Node::Node>) {
	// print to check AST structure
	let c = ast.len();
	println!{"\n[-] AST: "};
	for i in 0..c {
		if ast[i]._level == "Program".to_string() {
	    	// println!("{}: {}", ast[i]._level, ast[i]._name);
	    	println!("{} {}: {:?}", i, ast[i]._level, ast[i].to);
		} else if ast[i]._level == "Function".to_string() {
			 print!("  ");
	    	// println!("{}: {} {}", ast[i]._level, ast[i]._type, ast[i]._name);
	    	println!("{} {}: {:?}", i, ast[i]._level, ast[i].to);
		} else if ast[i]._level == "Statement".to_string() {
			 print!("    ");
	    	// println!("{}: {} {}", ast[i]._level, ast[i]._type, ast[i]._name);
	    	println!("{} {}: {:?}", i, ast[i]._level, ast[i].to);
		} else if ast[i]._level == "Expression".to_string() {
			 print!("      ");
	    	// println!("{}: {} {}", ast[i]._level, ast[i]._type, ast[i]._value);
	    	println!("{} {}: {:?}", i, ast[i]._level, ast[i].to);
		} else { println!("No Expression?");}
    }	
}

fn main() {
	// let filename_vec = vec!["missing_paren.c", "missing_retval.c", "no_brace.c", "no_semicolon.c", "no_space.c", "wrong_case.c"];
	// let filename_vec = vec!["multi_digit.c", "newlines.c", "no_newlines.c", "return_0.c", "return_2.c", "spaces.c"];
	// for filename in filename_vec {
		// let filename = "../../test/stage_1/valid/".to_owned() + filename;

		// let mut token_vec = lexer_lib::lex(&filename);

		let mut token_vec: Vec<Token::Token> = Vec::new();
		set_token(&mut token_vec);
		print_token(&token_vec);

		// first node of AST
		let mut ast: Vec<Node::Node> = Vec::new();
		println!("\n[+] Start parsing.\n");
		program(&mut token_vec,&mut ast);
		println!("\n[+] Finish parsing. Well done!");

		print_ast(&ast);
	// }
}


// level ::= <upper>{<lower>}
// type  ::= <upper>{<upper>}
// name  ::= [a-zA-Z] 


// level type name
fn program(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>) {
	ast.push(Node::new());
	ast[0]._level = String::from("Program"); 
	ast[0]._type  = String::from("FILE");
	ast[0]._name  = String::from("return_2"); 

	// print to test
	println!("{}: {}", &ast[0]._level, &ast[0]._name);
	
	function(&mut token_vec, &mut ast);
}

// level type name value
fn function(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>) {
	let ast_len = ast.len();
	if ast_len == 0 { panic!("Unable to set AST at program level."); }

	loop {
		
		// push ast node
		ast.push(Node::new());
		
		// add previous node's "to"
		ast[ast_len-1].to.push(ast_len);

		// set Function node's level
		ast[ast_len]._level = String::from("Function");
		
		// set Function node's type
		if token_vec[0]._type == "INT_KEYWORD" { 
			ast[ast_len]._type = String::from(token_vec[0]._type.clone());
			token_vec.remove(0);
		} else { panic!("Function type was wrong."); }

		// set Function node's name, value
		if token_vec[0]._type == "IDENTIFIER" {
			ast[ast_len]._name = String::from(token_vec[0]._value.clone());
			token_vec.remove(0);
		} else { panic!("Function name was wrong."); }
		
		// set Function node's (
		if token_vec[0]._type == "OPEN_PAREN" { 
			token_vec.remove(0);
		} else { panic!("Function ( was wrong."); }

		// set Function node's )
		if token_vec[0]._type == "CLOSE_PAREN" { 
			token_vec.remove(0);
		} else { panic!("Function ) was wrong."); }

		// set Function node's {
		if token_vec[0]._type == "OPEN_BRACE" { 
			token_vec.remove(0);
		} else { panic!("Function { was wrong."); }

		// print to test
		println!("  {}: {} {}", ast[ast_len]._level, ast[ast_len]._type, ast[ast_len]._name);

		statement(&mut token_vec, &mut ast);

		// set Function node's }
		if token_vec[0]._type == "CLOSE_BRACE" { 
			token_vec.remove(0);
			break;
		} else { panic!("Function }} was wrong.\n Function }}: {} {}", token_vec[0]._type, token_vec[0]._value); }
	}
}

// level type name
fn statement(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>) {
	let ast_len = ast.len();

	// push ast node
	ast.push(Node::new());

	// set previous node's "to"
	ast[ast_len-1].to.push(ast_len);

	// set Statement node's level
	ast[ast_len]._level = String::from("Statement");

	// set Statement node's type
	while token_vec[0]._type != "CLOSE_BRACE" {
		if token_vec[0]._type == "RETURN_KEYWORD" {
			ast[ast_len]._type = String::from(token_vec[0]._type.clone());
			ast[ast_len]._name = String::from(token_vec[0]._value.clone());
			token_vec.remove(0);

			println!("    {}: {} {}", ast[ast_len]._level, ast[ast_len]._type, ast[ast_len]._name);

			exp(&mut token_vec, &mut ast)
		} else { panic!("Statement type was wrong. \n Statement type: {} {}", token_vec[0]._type, token_vec[0]._value); }
		if token_vec[0]._type == "SEMICOLON" {
			token_vec.remove(0);
		} else { panic!("Statement end was wrong. \n Statement end: {} {}", token_vec[0]._type, token_vec[0]._value);		}
	}
}

//
fn exp(token_vec: &mut Vec<Token::Token>, ast: &mut Vec<Node::Node>) {
	let ast_len = ast.len();

	// push ast node
	ast.push(Node::new());

	// add previous node's "to"
	ast[ast_len-1].to.push(ast_len);

	// set Expression node's level
	ast[ast_len]._level = String::from("Expression");

	if token_vec[0]._type == "CONSTANT" {
		ast[ast_len]._type = String::from(token_vec[0]._type.clone());
		ast[ast_len]._value = String::from(token_vec[0]._value.clone());
		token_vec.remove(0);
	} else { panic!("Expression type was wrong. \n Expression type: {} {}", token_vec[0]._type, token_vec[0]._value); }

	println!("      {}: {} {}", ast[ast_len]._level, ast[ast_len]._type, ast[ast_len]._value);

}

// <program> ::= <function>
// <function> ::= <type> <identifier> "(" ")" "{" <statement> "}"
// <type> ::= "int"
// <identifier> ::= [a-zA-z_]{[a-zA-z_0-9]}
// <statement> ::= "return" <exp> ";"
// <exp> ::= [0-9]